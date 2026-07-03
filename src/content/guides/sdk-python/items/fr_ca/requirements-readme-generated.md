---
- Python >= 3.8

L'installation de base est pure-stdlib et fournit les utilitaires SSO. Le client API généré
(`DefaultApi`/`PublicApi`/`ModerationApi`) nécessite l'extra `client`, qui apporte
`urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0` et `typing-extensions >= 4.0.0` :

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```
---