---
Instaliraj iz oznake izdanja (ovaj projekt distribuira se putem git oznaka, a ne putem PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Za pristup REST s poslužitelja (pomoćni programi `admin()` / `public_api()`), dodaj `api` dodatak, koji učitava generirani klijent SDK-a:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Dodaj aplikaciju u `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---