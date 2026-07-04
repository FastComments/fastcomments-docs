---
Configurez votre tenant dans `settings.py` :

```python
import os

FASTCOMMENTS = {
    "TENANT_ID": os.environ.get("FASTCOMMENTS_TENANT_ID", "demo"),
}
```

Déposez le widget dans n'importe quel modèle :

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" %}
```
---