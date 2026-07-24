---
- Python >= 3.8

Базовата инсталация е чисто-stdlib и предоставя SSO инструменти. Генерираният API клиент (`DefaultApi`/`PublicApi`/`ModerationApi`) изисква допълнението `client`, което включва `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`, `pydantic >= 2.0.0` и `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```
---