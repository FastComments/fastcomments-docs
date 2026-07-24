- Python >= 3.8

Базовая установка является pure-stdlib и предоставляет утилиты SSO. Сгенерированный клиент API (`DefaultApi`/`PublicApi`/`ModerationApi`) требует дополнительный пакет `client`, который подтягивает `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`, `pydantic >= 2.0.0` и `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```