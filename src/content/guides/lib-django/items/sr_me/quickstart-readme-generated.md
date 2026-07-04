---
Konfigurišite vaš tenant u `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Ubacite widget u bilo koji šablon:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---