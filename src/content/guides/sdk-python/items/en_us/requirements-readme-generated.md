- Python >= 3.8

The base install is pure-stdlib and provides the SSO utilities. The generated
API client (`DefaultApi`/`PublicApi`/`ModerationApi`) needs the `client` extra,
which pulls in `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0`, and `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```