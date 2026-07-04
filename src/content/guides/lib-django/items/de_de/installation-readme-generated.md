---
Von einem Release‑Tag installieren (dieses Projekt wird über Git‑Tags verteilt, nicht über PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Für serverseitigen REST‑Zugriff (die `admin()` / `public_api()` Hilfsfunktionen), füge das
`api`‑Extra hinzu, das den vom SDK generierten Client einbindet:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Füge die App zu `INSTALLED_APPS` hinzu:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```