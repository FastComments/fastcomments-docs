---
Skonfiguruj swojego najemcę w `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Umieść widget w dowolnym szablonie:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---