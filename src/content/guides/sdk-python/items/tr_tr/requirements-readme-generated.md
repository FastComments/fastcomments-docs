---
- Python >= 3.8

Temel kurulum yalnızca pure-stdlib'dir ve SSO yardımcı programlarını sağlar. Oluşturulan API istemcisi (`DefaultApi`/`PublicApi`/`ModerationApi`) `client` ekini gerektirir; bu da `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`, `pydantic >= 2.0.0` ve `typing-extensions >= 4.0.0` paketlerini çeker:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```
---