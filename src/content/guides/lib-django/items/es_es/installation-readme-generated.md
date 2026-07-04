---
Instale desde una etiqueta de lanzamiento (este proyecto se distribuye mediante etiquetas git, no PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Para el acceso REST del lado del servidor (los asistentes `admin()` / `public_api()`), añada el extra `api`, que incorpora el cliente generado por el SDK:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Agregue la aplicación a `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---