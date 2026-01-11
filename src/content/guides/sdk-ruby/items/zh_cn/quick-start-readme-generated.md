### 使用已认证的 API（DefaultApi）

**重要：** 您必须在发起已认证的请求之前在 ApiClient 上设置您的 API 密钥。如果不这样做，请求将以 401 错误失败。

```ruby
require 'fastcomments-client'

# 创建并配置 API 客户端
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# 必填：设置您的 API 密钥（可在 FastComments 仪表板获取）
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# 使用已配置的客户端创建 API 实例
api = FastCommentsClient::DefaultApi.new(api_client)

# 现在您可以发起已认证的 API 调用
begin
  # 示例：添加 SSO 用户
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # 常见错误：
  # - 401：API 密钥缺失或无效
  # - 400：请求验证失败
end
```

### 使用公共 API（PublicApi）

公共端点不需要身份验证：

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

### 常见问题

1. **401 "missing-api-key" error**：确保在创建 DefaultApi 实例之前设置 `config.api_key['x-api-key'] = 'YOUR_KEY'`。
2. **Wrong API class**：服务器端已认证的请求请使用 `DefaultApi`，客户端/公共请求请使用 `PublicApi`。
3. **Null API key**：如果 API 密钥为 null，SDK 将静默跳过身份验证，导致 401 错误。