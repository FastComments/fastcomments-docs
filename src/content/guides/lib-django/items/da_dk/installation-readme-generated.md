---
Installer fra et release-tag (dette projekt distribueres via git-tags, ikke PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

For server-side REST-adgang (hjælperne `admin()` / `public_api()`), tilføj
`api` ekstra, som henter SDK'ens genererede klient:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Tilføj appen til `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---