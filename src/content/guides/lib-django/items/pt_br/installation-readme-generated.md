Instale a partir de uma tag de lançamento (este projeto é distribuído via tags git, não PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Para acesso REST do lado do servidor (os auxiliares `admin()` / `public_api()`), adicione o
`api` extra, que inclui o cliente gerado pelo SDK:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Adicione o aplicativo ao `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```