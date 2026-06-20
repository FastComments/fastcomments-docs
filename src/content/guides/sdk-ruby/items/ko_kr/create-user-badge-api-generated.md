## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_create_user_badge_response.rb)

## 예제

[inline-code-attrs-start title = 'create_user_badge 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # API 키 인증 구성: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키에 접두사를 설정하려면 다음 줄의 주석을 해제하세요. 예: 'Bearer' (기본값은 nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_user_badge_params = FastCommentsClient::CreateUserBadgeParams.new({user_id: 'user_id_example', badge_id: 'badge_id_example'}) # CreateUserBadgeParams | 

begin
  
  result = api_instance.create_user_badge(tenant_id, create_user_badge_params)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_user_badge: #{e}"
end
[inline-code-end]