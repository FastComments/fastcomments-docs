Install from a release tag (this project is distributed via git tags, not PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

For server-side REST access (the `admin()` / `public_api()` helpers), add the
`api` extra, which pulls in the SDK's generated client:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Add the app to `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```