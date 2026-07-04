---
從發行標籤安裝（此專案透過 git 標籤分發，而非 PyPI）：

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

若要進行伺服器端 REST 存取（`admin()` / `public_api()` 輔助函式），加入 `api` 額外套件，這會拉入 SDK 生成的客戶端：

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

將此應用程式加入 `INSTALLED_APPS`：

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---