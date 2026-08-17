---
Инсталирајте из ознаке издања (ов пројекат се дистрибуира преко git ознака, а не преко PyPI):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

За серверски REST приступ (помоћне функције `admin()` / `public_api()`), додајте `api` екстра, који увлачи генерисани клијент SDK-а:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

Додајте апликацију у `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---