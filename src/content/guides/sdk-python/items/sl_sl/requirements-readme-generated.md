---
- Python >= 3.8

Osnovna namestitev je čista standardna knjižnica in zagotavlja SSO pripomočke. Generiran API odjemalec (`DefaultApi`/`PublicApi`/`ModerationApi`) potrebuje dodatno komponento `client`, ki prinese `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0`, in `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```
---