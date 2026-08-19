---
Инсталирайте от етикет на версия (този проект се разпространява чрез git етикети, а не чрез PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

За достъп до REST от страна на сървъра (помощните функции `admin()` / `public_api()`), добавете `api` допълнението, което зарежда генерирания клиент на SDK:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Добавете приложението към `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---