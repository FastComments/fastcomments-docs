- Python >= 3.8

Базична инсталација је чиста стандардна библиотека и пружа SSO помагала. Генерисани
API клијент (`DefaultApi`/`PublicApi`/`ModerationApi`) захтева `client` екстра,
који увлачи `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0` и `typing-extensions >= 4.0.0`:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```