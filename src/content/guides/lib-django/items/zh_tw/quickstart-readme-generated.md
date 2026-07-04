---
在 `settings.py` 中配置您的租戶：

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

把小部件放入任意模板中：

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---