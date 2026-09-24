"""AMA-Bench BaseMethod wrapper around SelMem.

Drop this file into an AMA-Bench checkout as `src/method/selmem_method.py`
and register `selmem_c2`, `selmem_static`, `selmem_lastk` in
`src/method_register.py`. See README.md in this folder.

Construction writes a SELMEM1 vault via `examples/ama`.
Retrieve returns selected gists + living axioms — not a spoken answer.
The AMA-Bench LLM is the one that answers the question.
"""

from __future__ import annotations

import json
import os
import subprocess
import tempfile
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, List


def split_turns(traj_text: str) -> List[str]:
    documents: List[str] = []
    current: List[str] = []
    for line in traj_text.splitlines():
        s = line.lstrip()
        if s.startswith("Turn ") or s.startswith("Step "):
            if current:
                documents.append("\n".join(current))
                current = []
        current.append(line)
    if current:
        documents.append("\n".join(current))
    if not documents and traj_text.strip():
        documents = [traj_text]
    return documents


def pack_turns(turns: List[str], pack_words: int = 600) -> List[str]:
    packs: List[str] = []
    buf: List[str] = []
    words = 0
    for t in turns:
        w = len(t.split())
        if buf and words + w > pack_words:
            packs.append("\n\n".join(buf))
            buf, words = [], 0
        buf.append(t)
        words += w
    if buf:
        packs.append("\n\n".join(buf))
    return packs


@dataclass
class SelMemBook:
    mode: str
    packs: List[str]
    last_k: int
    book_path: str | None = None
    stats: dict = field(default_factory=dict)


class SelMemMethod:
    """Duck-typed BaseMethod. Does not import AMA-Bench so this tree stays standalone."""

    def __init__(
        self,
        mode: str = "c2",
        last_k: int = 8,
        pack_words: int = 600,
        ama_bin: str | None = None,
        config_path: str | None = None,
        embedding_engine: Any = None,
    ):
        self.mode = mode
        self.last_k = last_k
        self.pack_words = pack_words
        self.ama_bin = ama_bin or os.environ.get("SELMEM_AMA_BIN", "")
        self.embedding_engine = embedding_engine
        if config_path:
            raw = Path(config_path).read_text()
            for line in raw.splitlines():
                line = line.strip()
                if "=" in line and not line.startswith("#"):
                    k, v = line.split("=", 1)
                    if k.strip() == "last_k":
                        self.last_k = int(v.strip())
                    if k.strip() == "mode":
                        self.mode = v.strip()

    def memory_construction(self, traj_text: str, task: str = "") -> SelMemBook:
        text = traj_text
        if task.strip():
            text = f"## Task\n{task.strip()}\n\n## Trajectory\n{traj_text}"
        packs = pack_turns(split_turns(text), self.pack_words)
        book = SelMemBook(mode=self.mode, packs=packs, last_k=self.last_k)
        if self.mode == "lastk":
            book.stats = {"packs": len(packs), "kept": min(self.last_k, len(packs))}
            return book
        ama = self._ama_bin()
        dest = Path(tempfile.mkdtemp(prefix="selmem-ama-")) / "book.selmem"
        path = str(dest)
        book.book_path = path
        proc = subprocess.run(
            [
                ama,
                "construct",
                "--mode",
                self.mode,
                "--book",
                path,
                "--pack-words",
                str(self.pack_words),
            ],
            input=text,
            text=True,
            capture_output=True,
            check=False,
        )
        if proc.returncode != 0:
            raise RuntimeError(
                f"selmem ama construct failed ({proc.returncode}): {proc.stderr[-800:]}"
            )
        try:
            book.stats = json.loads(proc.stdout.strip().splitlines()[-1])
        except json.JSONDecodeError:
            book.stats = {"raw": proc.stdout[-400:]}
        return book

    def memory_retrieve(self, memory: SelMemBook, question: str) -> str:
        if not isinstance(memory, SelMemBook):
            raise TypeError("memory must be a SelMemBook")
        if memory.mode == "lastk":
            window = memory.packs[-memory.last_k :]
            return "\n\n".join(window) if window else "(empty last-k window)"
        if not memory.book_path:
            raise RuntimeError("C2/static book has no vault path")
        ama = self._ama_bin()
        proc = subprocess.run(
            [
                ama,
                "retrieve",
                "--book",
                memory.book_path,
                "--question",
                question,
                "--mode",
                memory.mode,
            ],
            text=True,
            capture_output=True,
            check=False,
        )
        if proc.returncode != 0:
            raise RuntimeError(
                f"selmem ama retrieve failed ({proc.returncode}): {proc.stderr[-800:]}"
            )
        return proc.stdout

    def _ama_bin(self) -> str:
        if self.ama_bin and Path(self.ama_bin).is_file():
            return self.ama_bin
        here = Path(__file__).resolve()
        crate = here.parents[2] if here.parent.name == "ama_bench" else Path.cwd()
        candidates = [
            crate / "target" / "release" / "examples" / "ama",
            Path("/tmp/selmem-ama"),
        ]
        for c in candidates:
            if c.is_file() and os.access(c, os.X_OK):
                return str(c)
        raise FileNotFoundError(
            "selmem ama binary not found. Build with "
            "`./run.sh build --release --example ama` and set SELMEM_AMA_BIN."
        )


class SelMemC2Method(SelMemMethod):
    def __init__(self, **kwargs: Any) -> None:
        kwargs.setdefault("mode", "c2")
        super().__init__(**kwargs)


class SelMemStaticMethod(SelMemMethod):
    def __init__(self, **kwargs: Any) -> None:
        kwargs.setdefault("mode", "static")
        super().__init__(**kwargs)


class SelMemLastKMethod(SelMemMethod):
    def __init__(self, **kwargs: Any) -> None:
        kwargs.setdefault("mode", "lastk")
        super().__init__(**kwargs)
