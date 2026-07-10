---
- Python >= 3.8

基礎安裝是純標準庫，並提供 SSO 工具。產生的 API 客戶端（`DefaultApi`/`PublicApi`/`ModerationApi`）需要 `client` 額外套件，該套件會拉入 `urllib3 >= 1.25.3`、`python-dateutil >= 2.8.2`、`pydantic >= 2.0.0` 以及 `typing-extensions >= 4.0.0`：

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```
---