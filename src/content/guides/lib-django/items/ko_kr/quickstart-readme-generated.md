`settings.py`에서 테넌트를 구성합니다:

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

위젯을 모든 템플릿에 삽입합니다:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```