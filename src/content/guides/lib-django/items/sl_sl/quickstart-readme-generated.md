---
Konfigurirajte svoj najemnik v `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Vstavite widget v katerokoli predlogo:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```