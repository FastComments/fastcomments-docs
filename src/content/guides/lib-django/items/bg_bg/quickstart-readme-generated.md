---
Конфигурирайте вашия тенант в `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Поставете уиджета във всеки шаблон:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---