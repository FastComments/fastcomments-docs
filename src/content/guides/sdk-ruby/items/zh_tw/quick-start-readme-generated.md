### 使用已驗證的 API (DefaultApi)

**重要：** 您必須在發出已驗證請求之前，在 ApiClient 上設定您的 API 金鑰。如果未設定，請求會以 401 錯誤失敗。

```ruby
require 'fastcomments-client'

# 建立並配置 API 用戶端
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# 必須：在此設定您的 API 金鑰（從 FastComments 儀表板取得）
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# 使用已配置的用戶端建立 API 實例
api = FastCommentsClient::DefaultApi.new(api_client)

# 現在您可以進行已驗證的 API 呼叫
begin
  # 範例：新增 SSO 使用者
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
require 'fastcomments-client'

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

### 常見問題

1. **401 "missing-api-key" error**：請在建立 `DefaultApi` 實例之前確認已設定 `config.api_key['x-api-key'] = 'YOUR_KEY'`。
2. **Wrong API class**：伺服器端的已驗證請求請使用 `DefaultApi`，用於客戶端/公開請求請使用 `PublicApi`。
3. **Null API key**：若 API 金鑰為 null，SDK 會靜默跳過驗證，導致 401 錯誤。