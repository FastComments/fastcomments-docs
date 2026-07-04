Встановіть з тегу випуску (цей проєкт розповсюджується через git‑теги, а не через PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Для серверного доступу до REST (використовуючи допоміжні функції `admin()` / `public_api()`), додайте додатковий параметр `api`, який підключає згенерований клієнт SDK:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Додайте застосунок до `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```