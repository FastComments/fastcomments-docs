---
Configure your tenant in `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Drop the widget into any template:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---