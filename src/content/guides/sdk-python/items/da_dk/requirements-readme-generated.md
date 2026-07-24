---
- Python >= 3.8

Den grundlæggende installation er ren standardbibliotek og leverer SSO‑værktøjerne. Den genererede API‑klient (`DefaultApi`/`PublicApi`/`ModerationApi`) kræver `client`‑ekstraen, som henter `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`, `pydantic >= 2.0.0` og `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```
---