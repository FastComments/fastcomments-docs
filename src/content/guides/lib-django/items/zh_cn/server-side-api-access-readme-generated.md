With the `[api]` extra installed, call the FastComments REST API through the SDK,
pre-configured with your API key and region:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # 已认证 (DefaultApi)
public_api().get_comments_public(...)            # 公共 (PublicApi)

# 为 API 调用或客户端交接生成 SSO 令牌：
token = get_manager().sso().token_for(request.user)
```