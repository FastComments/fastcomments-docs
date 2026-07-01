### 使用已驗證的 API（DefaultApi）

**重要：** 您必須在 Configuration 中設定 API 金鑰，才能發出已驗證的請求。如果未設定，請求將會以 401 錯誤失敗。

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# 建立並設定 API 客戶端
config = Configuration()
config.host = "https://fastcomments.com/api"

# 必要：設定您的 API 金鑰（從 FastComments 儀表板取得）
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# 使用已設定的客戶端建立 API 實例
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# 現在您可以發出已驗證的 API 呼叫
try:
    # 範例：新增 SSO 使用者
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # 常見錯誤：
    # - 401：缺少或無效的 API 金鑰
    # - 400：請求驗證失敗
```

### 使用公開 API（PublicApi）

公開端點不需要驗證：

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### 使用審查儀表板（ModerationApi）

`ModerationApi` 為審查員儀表板提供功能。方法會以傳入 `sso` 令牌的方式代表審查員呼叫：

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # 計算等待審查的評論
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### 使用 SSO（單一登入）

SDK 包含用於產生安全 SSO 令牌的工具：

```python
from sso import FastCommentsSSO, SecureSSOUserData

# 建立使用者資料
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# 使用您的 API 密鑰建立 SSO 實例
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# 產生 SSO 令牌
sso_token = sso.create_token()

# 在前端使用此令牌或傳遞給 API 呼叫
print(f"SSO Token: {sso_token}")
```

簡易 SSO（較不安全，僅供測試）：

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### 常見問題

1. **401 "missing-api-key" 錯誤**：確保在建立 DefaultApi 實例之前設定 `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}`。
2. **錯誤的 API 類別**：對於伺服器端已驗證的請求使用 `DefaultApi`，對於客戶端/公開請求使用 `PublicApi`，而審查員儀表板請求則使用 `ModerationApi`。
3. **匯入錯誤**：確保從正確的模組匯入：
   - API 客戶端：`from client import ...`
   - SSO 工具：`from sso import ...`