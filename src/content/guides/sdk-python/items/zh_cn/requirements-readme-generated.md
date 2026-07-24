- Python >= 3.8

基础安装是纯标准库的，并提供 SSO 实用工具。生成的 API 客户端（`DefaultApi`/`PublicApi`/`ModerationApi`）需要 `client` 额外依赖，它会拉取 `urllib3 >= 1.25.3`、`python-dateutil >= 2.8.2`、`pydantic >= 2.0.0` 和 `typing-extensions >= 4.0.0`：

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```