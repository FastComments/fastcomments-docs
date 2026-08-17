---
Installeer vanaf een release‑tag (dit project wordt gedistribueerd via git‑tags, niet via PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Voor server‑side REST‑toegang (de `admin()` / `public_api()` helpers), voeg de
`api` extra toe, die de door de SDK gegenereerde client binnenhaalt:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Voeg de app toe aan `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---