### 使用已认证的 API（DefaultApi）

**重要提示：** 在进行已认证请求之前，必须在 ApiClient 上设置您的 API 密钥。如果未设置，请求将返回 401 错误。

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
  # - 401：缺少或无效的 API 密钥
  # - 400：请求验证失败
end
```

### 使用公共 API（PublicApi）

公共端点无需认证：

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

### 使用审查 API（ModerationApi）

审查方法为审查员仪表板提供功能。请传入 `sso` 令牌，以便代表已使用 SSO 认证的审查员发起请求：

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

### 常见问题

1. **401 “missing-api-key” 错误**：请确保在创建 DefaultApi 实例之前设置 `config.api_key['x-api-key'] = 'YOUR_KEY'`。
2. **错误的 API 类**：对服务器端已认证请求使用 `DefaultApi`，对客户端/公共请求使用 `PublicApi`，对审查员仪表板请求使用 `ModerationApi`。
3. **API 密钥为空**：如果 API 密钥为 null，SDK 将静默跳过认证，导致 401 错误。