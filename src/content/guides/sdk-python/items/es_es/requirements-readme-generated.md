---
- Python >= 3.8

La instalación base es pure-stdlib y proporciona las utilidades SSO.  
El cliente API generado (`DefaultApi`/`PublicApi`/`ModerationApi`) necesita el extra `client`, que incluye `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`, `pydantic >= 2.0.0` y `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```
---