Instalirajte s oznakom izdanja (ovaj projekt se distribuira putem git oznaka, a ne putem PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Za REST pristup na serveru (pomoćne funkcije `admin()` / `public_api()`), dodajte dodatak `api`, koji učitava generisanog klijenta SDK‑a:

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