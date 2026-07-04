使用已安裝的 `[api]` 擴充功能，可透過 SDK 呼叫 FastComments REST API，已預先配置您的 API 金鑰與區域：

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # 已驗證 (DefaultApi)
public_api().get_comments_public(...)            # 公開 (PublicApi)

# 為 API 呼叫或客戶端交接產生 SSO 令牌：
token = get_manager().sso().token_for(request.user)
```