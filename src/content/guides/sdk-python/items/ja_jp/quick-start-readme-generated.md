### 認証された API の使用 (DefaultApi)

**重要:** 認証されたリクエストを行う前に必ず Configuration に API キーを設定してください。設定しないと、リクエストは 401 エラーで失敗します。

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com/api"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Create the API instance with the configured client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Now you can make authenticated API calls
try:
    # Example: Add an SSO user
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Common errors:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### パブリック API の使用 (PublicApi)

パブリックエンドポイントは認証を必要としません：

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public(
        tenant_id="YOUR_TENANT_ID",
        url_id="page-url-id"
    )
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO（シングルサインオン）の使用

SDK には安全な SSO トークンを生成するユーティリティが含まれています：

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Create SSO instance with your API secret
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generate the SSO token
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
print(f"SSO Token: {sso_token}")
```

簡易 SSO（セキュリティが低く、テスト用）：

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### よくある問題

1. **401 "missing-api-key" エラー**: `DefaultApi` インスタンスを作成する前に `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` を設定していることを確認してください。
2. **API クラスの間違い**: サーバー側の認証済みリクエストには `DefaultApi` を、クライアント側／パブリックリクエストには `PublicApi` を使用してください。
3. **インポートエラー**: 正しいモジュールからインポートしていることを確認してください：
   - API クライアント: `from client import ...`
   - SSO ユーティリティ: `from sso import ...`