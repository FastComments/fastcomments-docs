---
Configure su inquilino en `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Coloque el widget en cualquier plantilla:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---