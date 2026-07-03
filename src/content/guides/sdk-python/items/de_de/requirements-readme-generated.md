---
- Python >= 3.8

Die Basissinstallation ist pure-stdlib und liefert die SSO‑Dienstprogramme. Der generierte
API-Client (`DefaultApi`/`PublicApi`/`ModerationApi`) benötigt das `client`‑Extra,
das `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0` und `typing-extensions >= 4.0.0` einbindet:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```
---