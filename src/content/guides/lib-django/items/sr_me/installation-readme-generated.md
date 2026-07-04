---
Instalirajte iz tag‑a za izdanje (ovaj projekat se distribuira putem git tagova, a ne putem PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Za server‑side REST pristup (pomoćnice `admin()` / `public_api()`), dodajte
`api` dodatak, koji učitava generisani klijent SDK‑a:

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