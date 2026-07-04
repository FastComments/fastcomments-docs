从发布标签安装（此项目通过 git 标签分发，而不是 PyPI）：

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

对于服务器端 REST 访问（`admin()` / `public_api()` 辅助函数），添加 `api` 额外项，它会引入 SDK 生成的客户端：

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

将该应用添加到 `INSTALLED_APPS`：

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```