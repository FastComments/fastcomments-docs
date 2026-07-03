---
- Python >= 3.8

L'installazione di base è pure-stdlib e fornisce le utility SSO. Il client API generato (`DefaultApi`/`PublicApi`/`ModerationApi`) richiede l'extra `client`, che include `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`, `pydantic >= 2.0.0` e `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```
---