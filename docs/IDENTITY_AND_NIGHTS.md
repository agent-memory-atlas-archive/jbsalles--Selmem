# Direct identity, people and remembered nights

The organ distinguishes what it **is** from what it **remembers**.

## Constitutive identity

`EntityProfile.name` is the agent's own stable name, delivered as a direct
role anchor in its system prompt. It is not an inferred axiom, a memory of
someone else's introduction or a recalled episode. Night, recall, suppression
and book reset cannot silently turn the agent's name into the human's name.
Changing the agent's own name requires an explicit profile change.

## Remembered human names

An explicit introduction such as `Je m'appelle Victor` is admitted on sleep as
an attributed `External` **Selfhood** episode under `interlocutor-name`.
It passes the ordinary admission gate and may drift, become latent, or be
forgotten. The agent must not infer the human's name from its own name.

## Nights of sleep

Each UI `/sleep` offers a separate `Internal` **Selfhood** episode to the
ordinary gate, *after* sleep has run. No exact lifetime night counter is sent
to the model. When asked, the agent is offered only the number of currently
accessible, unsuppressed active sleep episodes as a **lower bound**. Other
night episodes may have been forgotten, so this is not an exact elapsed-day
calculation. Legacy nights cannot be retrospectively reconstructed.

The public demo additionally expires RAM-only visitor sessions after 24 hours;
expiration destroys their entire memory and cannot be undone by a prompt.
