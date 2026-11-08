`settings.py` में अपने टेनेंट को कॉन्फ़िगर करें:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

विजेट को किसी भी टेम्पलेट में जोड़ें:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```