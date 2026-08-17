---
Zainstaluj z tagu wydania (ten projekt jest dystrybuowany za pośrednictwem tagów git, a nie PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Aby uzyskać dostęp do REST po stronie serwera (pomocnicze funkcje `admin()` / `public_api()`), dodaj
`api` extra, który pobiera wygenerowanego klienta SDK:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Dodaj aplikację do `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---