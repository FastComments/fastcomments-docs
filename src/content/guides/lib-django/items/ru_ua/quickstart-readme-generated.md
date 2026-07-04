---
Настройте ваш тенант в `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Поместите виджет в любой шаблон:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---