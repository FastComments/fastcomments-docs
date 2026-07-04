---
Налаштуйте ваш тенант у `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Вставте віджет у будь‑який шаблон:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---