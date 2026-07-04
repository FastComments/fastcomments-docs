עם ההרחבה `[api]` מותקנת, קראו ל‑REST API של FastComments דרך ה‑SDK, המוגדר מראש עם מפתח ה‑API והאזור שלכם:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # מאומת (DefaultApi)
public_api().get_comments_public(...)            # ציבורי (PublicApi)

# יצירת אסימון SSO לשיחות API או העברת לקוח:
token = get_manager().sso().token_for(request.user)
```