---
- Python >= 3.8

Osnovna instalacija je čista standardna biblioteka i pruža SSO alate. Generisani
API klijent (`DefaultApi`/`PublicApi`/`ModerationApi`) zahteva `client` dodatak,
koji povlači `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0`, i `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```
---