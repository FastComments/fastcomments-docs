Configure seu tenant em `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Insira o widget em qualquer template:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```