---
- Python >= 3.8

De basisinstallatie is pure-stdlib en levert de SSO‑hulpmiddelen. De gegenereerde
API‑client (`DefaultApi`/`PublicApi`/`ModerationApi`) heeft de extra `client` nodig,
die `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0` en `typing-extensions >= 4.0.0` binnenhaalt:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```
---