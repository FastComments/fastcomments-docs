---
Configureer uw tenant in `settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Plaats de widget in een willekeurige template:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---