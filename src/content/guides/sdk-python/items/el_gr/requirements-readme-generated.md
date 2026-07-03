- Python >= 3.8

Η βασική εγκατάσταση είναι pure-stdlib και παρέχει τις βοηθητικές λειτουργίες SSO. Ο παραγόμενος
API client (`DefaultApi`/`PublicApi`/`ModerationApi`) χρειάζεται το `client` extra,
που φέρνει το `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0` και `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```