---
Konfigurer din tenant i `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Indsæt widget'en i enhver skabelon:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---