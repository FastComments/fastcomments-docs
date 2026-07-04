---
Με το πρόσθετο `[api]` εγκατεστημένο, καλέστε το FastComments REST API μέσω του SDK, προ‑ρυθμισμένο με το κλειδί API και την περιοχή σας:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # αυθεντικοποιημένο (DefaultApi)
public_api().get_comments_public(...)            # δημόσιο (PublicApi)

# Δημιουργήστε ένα token SSO για κλήσεις API ή παράδοση πελάτη:
token = get_manager().sso().token_for(request.user)
```
---