---
- Python >= 3.8

ベースインストールは純粋な標準ライブラリで、SSOユーティリティを提供します。生成された
APIクライアント（`DefaultApi`/`PublicApi`/`ModerationApi`）は `client` エクストラが必要で、これにより `urllib3 >= 1.25.3`、`python-dateutil >= 2.8.2`、`pydantic >= 2.0.0`、および `typing-extensions >= 4.0.0` がインストールされます：

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0"
```
---