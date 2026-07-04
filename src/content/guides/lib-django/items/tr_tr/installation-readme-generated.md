---
Bir sürüm etiketinden kurun (bu proje PyPI yerine git etiketleriyle dağıtılır):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Sunucu tarafı REST erişimi için (`admin()` / `public_api()` yardımcıları), SDK'nın oluşturduğu istemciyi çeken `api` ekini ekleyin:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

`INSTALLED_APPS` içine uygulamayı ekleyin:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---