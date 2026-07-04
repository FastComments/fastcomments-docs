Konfigurieren Sie Ihren Mandanten in `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Fügen Sie das Widget in jede Vorlage ein:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```