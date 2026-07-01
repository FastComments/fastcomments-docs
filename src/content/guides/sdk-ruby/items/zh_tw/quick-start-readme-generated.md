### 使用已驗證的 APIs（DefaultApi）

**重要：** 在執行已驗證的請求之前，必須在 ApiClient 上設定您的 API 金鑰。若未設定，請求將會以 401 錯誤失敗。

```ruby
require 'fastcomments'

# Create and configure the API client
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Create the API instance with the configured client
api = FastCommentsClient::DefaultApi.new(api_client)

# Now you can make authenticated API calls
begin
  # Example: Add an SSO user
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Common errors:
  # - 401: API key is missing or invalid
  # - 400: Request validation failed
end
```

### 使用公開 APIs（PublicApi）

公開端點不需要驗證：

```ruby
require 'fastcomments'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    'YOUR_TENANT_ID',
    'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### 使用審核 APIs（ModerationApi）

審核方法為審核員儀表板提供功能。傳遞 `sso` 令牌，使請求代表已 SSO 驗證的審核員執行：

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Example: List comments in the moderation queue
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### 常見問題

1. **401 “missing-api-key” 錯誤**：在建立 DefaultApi 實例之前，請確保已設定 `config.api_key['x-api-key'] = 'YOUR_KEY'`。
2. **錯誤的 API 類別**：對於伺服器端已驗證的請求使用 `DefaultApi`，對於客戶端/公開請求使用 `PublicApi`，以及對於審核員儀表板的請求使用 `ModerationApi`。
3. **API 金鑰為 null**：如果 API 金鑰為 null，SDK 會默默跳過驗證，導致 401 錯誤。