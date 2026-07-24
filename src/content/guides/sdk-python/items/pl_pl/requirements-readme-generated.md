---
- Python >= 3.8

Podstawowa instalacja jest czystą biblioteką standardową i zapewnia narzędzia SSO. Wygenerowany
klient API (`DefaultApi`/`PublicApi`/`ModerationApi`) wymaga dodatkowego pakietu `client`,
który pobiera `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0` oraz `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```
---