### 使用已认证的 API (DefaultApi)

**重要：** 在发起需要认证的请求之前，您必须在 ApiClient 上设置您的 API 密钥。如果不设置，请求会返回 401 错误。

```ruby
require 'fastcomments'

# 创建并配置 API 客户端
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# 必需：设置您的 API 密钥（从 FastComments 仪表板获取）
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# 使用已配置的客户端创建 API 实例
api = FastCommentsClient::DefaultApi.new(api_client)

# 现在您可以发起带认证的 API 调用
begin
  # 示例：添加一个 SSO 用户
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

### 使用公共 API (PublicApi)

公共端点不需要认证：

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

### 使用审核 API (ModerationApi)

审核方法为审核者仪表板提供功能。传入 `sso` 令牌，使请求以经过 SSO 认证的审核者身份发出：

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # 示例：列出审核队列中的评论
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### 常见问题

1. **401 "missing-api-key" 错误**：在创建 DefaultApi 实例之前，请确保已设置 `config.api_key['x-api-key'] = 'YOUR_KEY'`。
2. **错误的 API 类**：对服务器端的带认证请求使用 `DefaultApi`，对客户端/公共请求使用 `PublicApi`，对审核者仪表板请求使用 `ModerationApi`。
3. **API 密钥为 null**：如果 API 密钥为 null，SDK 将静默跳过认证，导致 401 错误。