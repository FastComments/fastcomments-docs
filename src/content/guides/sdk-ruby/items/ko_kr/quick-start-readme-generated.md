---
### 인​증​된​ API​ 사용 (DefaultApi)

**중요:** 인증된 요청을 만들기 전에 ApiClient에 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류와 함께 실패합니다.

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

### 공개​ API​ 사용 (PublicApi)

공개 엔드포인트는 인증이 필요하지 않습니다:

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

### 중재​ API​ 사용 (ModerationApi)

중재 메서드는 모더레이터 대시보드를 구동합니다. `sso` 토큰을 전달하면 요청이 SSO 인증된 모더레이터를 대신해 수행됩니다:

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

### 일반적인 문제

1. **401 "missing-api-key" 오류**: DefaultApi 인스턴스를 만들기 전에 `config.api_key['x-api-key'] = 'YOUR_KEY'`를 설정했는지 확인하십시오.
2. **잘못된 API 클래스**: 서버 측 인증 요청에는 `DefaultApi`를, 클라이언트 측/공개 요청에는 `PublicApi`를, 모더레이터 대시보드 요청에는 `ModerationApi`를 사용하세요.
3. **Null API 키**: API 키가 null이면 SDK가 인증을 조용히 건너뛰게 되며, 이로 인해 401 오류가 발생합니다.
---