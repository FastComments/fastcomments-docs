---
Konfigurirajte svoj tenant u `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Umetnite widget u bilo koji predložak:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---