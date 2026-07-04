`[api]` エクストラがインストールされている状態で、SDK を通じて FastComments REST API を呼び出します。API キーとリージョンが事前に設定されています：

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # 認証済み (DefaultApi)
public_api().get_comments_public(...)            # 公開 (PublicApi)

# API 呼び出しまたはクライアントハンドオフ用に SSO トークンを生成:
token = get_manager().sso().token_for(request.user)
```