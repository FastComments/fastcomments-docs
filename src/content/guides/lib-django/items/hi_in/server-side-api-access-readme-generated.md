`[api]` एक्स्ट्रा इंस्टॉल होने पर, FastComments REST API को SDK के माध्यम से कॉल करें,
जो आपकी API key और region के साथ पहले से कॉन्फ़िगर है:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # authenticated (DefaultApi)
public_api().get_comments_public(...)            # public (PublicApi)

# API कॉल या क्लाइंट हैंड-ऑफ़ के लिए SSO टोकन जनरेट करें:
token = get_manager().sso().token_for(request.user)
```