## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 경로 | 예 |  |
| postIds | array | 쿼리 | 아니오 |  |
| sso | string | 쿼리 | 아니오 |  |

## 응답

반환: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_reacts_public200_response.rb)

## 예제

[inline-code-attrs-start title = 'get_user_reacts_public 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  post_ids: ['inner_example'], # Array<String> | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_user_reacts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_reacts_public: #{e}"
end
[inline-code-end]

---