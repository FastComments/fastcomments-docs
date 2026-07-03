- Python >= 3.8

A instalação base é puro-stdlib e fornece as utilidades SSO. O cliente API gerado (`DefaultApi`/`PublicApi`/`ModerationApi`) precisa do extra `client`, que traz `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`, `pydantic >= 2.0.0` e `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```