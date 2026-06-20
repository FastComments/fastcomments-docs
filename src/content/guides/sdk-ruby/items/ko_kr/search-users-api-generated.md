## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| usernameStartsWith | string | query | 아니요 |  |
| mentionGroupIds | array | query | 아니요 |  |
| sso | string | query | 아니요 |  |
| searchSection | string | query | 아니요 |  |

## 응답

반환: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/search_users_result.rb)

## 예제

[inline-code-attrs-start title = 'search_users 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
url_id = 'url_id_example' # 문자열 | 
opts = {
  username_starts_with: 'username_starts_with_example', # 문자열 | 
  mention_group_ids: ['inner_example'], # Array<String> | 
  sso: 'sso_example', # 문자열 | 
  search_section: 'fast' # 문자열 | 
}

begin
  
  result = api_instance.search_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->search_users: #{e}"
end
[inline-code-end]

---