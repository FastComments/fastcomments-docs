Installez à partir d’un tag de version (ce projet est distribué via des tags git, pas PyPI) :

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Pour un accès REST côté serveur (les aides `admin()` / `public_api()`), ajoutez l’extra
`api`, qui inclut le client généré du SDK :

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Ajoutez l’application à `INSTALLED_APPS` :

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```