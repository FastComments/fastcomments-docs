---
- Python >= 3.8

ההתקנה הבסיסית היא pure-stdlib ומספקת את כלי ה‑SSO. לקוח ה‑API שנוצר (`DefaultApi`/`PublicApi`/`ModerationApi`) דורש את ה‑extra `client`, שמוסיף את `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`, `pydantic >= 2.0.0` ו‑`typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```
---