Установите из метки релиза (этот проект распространяется через git‑теги, а не через PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Для доступа к REST‑интерфейсу на стороне сервера (вспомогательные функции `admin()` / `public_api()`), добавьте дополнительный параметр `api`, который подтягивает сгенерированный клиент SDK:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Добавьте приложение в `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```