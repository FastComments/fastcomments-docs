## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| id | string | query | 예 |  |

## 응답

반환: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_v2_page_react_users_response.rb)

## 예제

[inline-code-attrs-start title = 'get_v2_page_react_users 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_v2_page_react_users(tenant_id, url_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_v2_page_react_users: #{e}"
end
[inline-code-end]

---