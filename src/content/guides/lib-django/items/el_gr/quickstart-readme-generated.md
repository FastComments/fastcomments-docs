---
Διαμορφώστε το ενοικιαστή σας στο `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Τοποθετήστε το widget σε οποιοδήποτε πρότυπο:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---