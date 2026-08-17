---
リリースタグからインストールします（このプロジェクトは PyPI ではなく git タグで配布されています）:

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

サーバーサイドの REST アクセス（`admin()` / `public_api()` ヘルパー）を使用するには、`api` エクストラを追加します。これにより SDK が生成したクライアントが取り込まれます:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

アプリを `INSTALLED_APPS` に追加します:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---