---
- Python >= 3.8

Базова установка є чистою стандартною бібліотекою і надає утиліти SSO. Згенерований
API‑клієнт (`DefaultApi`/`PublicApi`/`ModerationApi`) потребує додаткового `client`,
який підтягує `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0`, та `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```
---