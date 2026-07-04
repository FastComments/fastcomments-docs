---
`settings.py`でテナントを構成します:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

任意のテンプレートにウィジェットを配置します:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---