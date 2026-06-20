### 인증된 API 사용 (DefaultApi)

**중요:** 인증된 요청을 하기 전에 ApiClient에서 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류로 실패합니다.

```ruby
require 'fastcomments'

# API 클라이언트를 생성하고 구성합니다
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# 필수: API 키를 설정하세요 (FastComments 대시보드에서 가져옵니다)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# 구성된 클라이언트로 API 인스턴스를 생성합니다
api = FastCommentsClient::DefaultApi.new(api_client)

# 이제 인증된 API 호출을 할 수 있습니다
begin
  # 예: SSO 사용자 추가
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # 일반적인 오류:
  # - 401: API 키가 없거나 잘못되었습니다
  # - 400: 요청 유효성 검사 실패
end
```

### 공개 API 사용 (PublicApi)

공개 엔드포인트는 인증이 필요하지 않습니다:

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

### 모더레이션 API 사용 (ModerationApi)

모더레이션 메서드는 모더레이터 대시보드를 구동합니다. 요청이 SSO로 인증된 모더레이터를 대신하여 이루어지도록 `sso` 토큰을 전달하세요:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # 예: 모더레이션 큐의 댓글 목록
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### 일반적인 문제

1. **401 "missing-api-key" 오류**: DefaultApi 인스턴스를 생성하기 전에 `config.api_key['x-api-key'] = 'YOUR_KEY'`를 설정했는지 확인하세요.
2. **잘못된 API 클래스**: 서버 측 인증된 요청에는 `DefaultApi`를, 클라이언트 측/공개 요청에는 `PublicApi`를, 모더레이터 대시보드 요청에는 `ModerationApi`를 사용하세요.
3. **API 키가 null인 경우**: API 키가 null이면 SDK가 인증을 조용히 건너뛰어 401 오류가 발생합니다.