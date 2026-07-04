---
在 `settings.py` 中配置您的租户：

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

将小部件放入任意模板：

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---