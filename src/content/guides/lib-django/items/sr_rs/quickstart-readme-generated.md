---
Подесите вашег тенанта у `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Убаците виџет у било који шаблон:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```