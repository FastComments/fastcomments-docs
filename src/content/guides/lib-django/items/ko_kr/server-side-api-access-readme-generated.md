`[api]` 추가 기능이 설치된 경우, SDK를 통해 FastComments REST API를 호출하십시오,  
API 키와 지역이 사전 구성되어 있습니다:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # authenticated (DefaultApi)
public_api().get_comments_public(...)            # public (PublicApi)

# Generate an SSO token for API calls or client hand-off:
token = get_manager().sso().token_for(request.user)
```