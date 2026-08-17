---
Installa da un tag di rilascio (questo progetto è distribuito tramite tag git, non PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Per l'accesso REST lato server (i helper `admin()` / `public_api()`), aggiungi il
`api` extra, che importa il client generato dall'SDK:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Aggiungi l'app a `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---