---
Instalirajte sa oznakom izdanja (ovaj projekat se distribuira preko git oznaka, a ne putem PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Za pristup REST-u na serveru (pomoćnici `admin()` / `public_api()`), dodajte `api` dodatak, koji učitava generisanog klijenta SDK-a:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Dodajte aplikaciju u `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---