---
Kiracınızı `settings.py` içinde yapılandırın:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Widget'ı herhangi bir şablona ekleyin:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---