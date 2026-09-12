use selmem::Config;

#[test]
fn parses_plain_and_selmem_prefixed_keys() {
    let cfg = Config::parse(
        r#"
# comment
llm=https://api.x.ai/v1/chat/completions
SELMEM_MODEL="grok-4.3"
export SELMEM_API_KEY=xai-test
temp: 0
reasoning=none
"#,
    );
    assert_eq!(
        cfg.file("llm"),
        Some("https://api.x.ai/v1/chat/completions")
    );
    assert_eq!(cfg.file("model"), Some("grok-4.3"));
    assert_eq!(cfg.file("api_key"), Some("xai-test"));
    assert_eq!(cfg.file("temp"), Some("0"));
    assert_eq!(cfg.file("reasoning"), Some("none"));
}

#[test]
fn skips_vault_magic_lines() {
    let cfg = Config::parse("SELMEM1\nllm=http://127.0.0.1:11434/v1/chat/completions\n");
    assert_eq!(
        cfg.file("llm"),
        Some("http://127.0.0.1:11434/v1/chat/completions")
    );
}

#[test]
fn load_path_roundtrip() {
    let dir = std::env::temp_dir().join(format!("selmem-cfg-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("opts.selmem");
    std::fs::write(&path, "model=llama3\nhttp_timeout=12\n").unwrap();
    let cfg = Config::load_path(&path).unwrap();
    assert_eq!(cfg.file("model"), Some("llama3"));
    assert_eq!(cfg.file("http_timeout"), Some("12"));
    let _ = std::fs::remove_dir_all(&dir);
}
