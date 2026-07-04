Configura il tuo tenant in `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Inserisci il widget in qualsiasi template:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```