---
הגדר את השוכר שלך ב-`settings.py`:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

הוסף את הווידג'ט לכל תבנית:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---