---
- Python >= 3.8

ベースインストールは pure-stdlib で、SSO ユーティリティを提供します。生成された
API クライアント（`DefaultApi`/`PublicApi`/`ModerationApi`）は `client` エクストラが必要で、
`urllib3 >= 1.25.3`、`python-dateutil >= 2.8.2`、`pydantic >= 2.0.0`、および `typing-extensions >= 4.0.0` を取得します:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```
---