### 使用已驗證的 API (DefaultApi)

**重要：** 在進行需驗證的請求之前，您必須在 ApiClient 上設定您的 API 金鑰。如果未設定，請求將會以 401 錯誤失敗。

```ruby
require 'fastcomments'

# 建立並設定 API 用戶端
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# 使用已配置的用戶端建立 API 實例
api = FastCommentsClient::DefaultApi.new(api_client)

# 現在你就可以進行需驗證的 API 呼叫
begin
  # 範例：新增一個 SSO 使用者
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # 常見錯誤：
  # - 401：API 金鑰遺失或無效
  # - 400：請求驗證失敗
end
```

### 使用公開 API (PublicApi)

公開端點不需要驗證：

```ruby
require 'fastcomments'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    tenant_id: 'YOUR_TENANT_ID',
    url_id: 'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### 使用審核 API (ModerationApi)

審核方法為管理員儀表板提供功能。傳遞一個 `sso` token，使請求以 SSO 驗證的管理員身份發出：

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # 範例：列出審核佇列中的評論
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### 常見問題

1. **401 "missing-api-key" 錯誤**：請確保在建立 DefaultApi 實例之前，已設定 `config.api_key['x-api-key'] = 'YOUR_KEY'`。
2. **錯誤的 API 類別**：伺服端需驗證的請求請使用 `DefaultApi`，用戶端/公開請求請使用 `PublicApi`，管理員儀表板請使用 `ModerationApi`。
3. **API 金鑰為 null**：若 API 金鑰為 null，SDK 會悄悄略過驗證，導致 401 錯誤。
---