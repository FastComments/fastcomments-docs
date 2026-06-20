---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| usernameStartsWith | string | query | いいえ |  |
| mentionGroupIds | array | query | いいえ |  |
| sso | string | query | いいえ |  |
| searchSection | string | query | いいえ |  |

## レスポンス

戻り値: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/search_users_result.rb)

## 例

[inline-code-attrs-start title = 'search_users の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
opts = {
  username_starts_with: 'username_starts_with_example', # String | 
  mention_group_ids: ['inner_example'], # Array<String> | 
  sso: 'sso_example', # String | 
  search_section: 'fast' # String | 
}

begin
  
  result = api_instance.search_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->search_users: #{e}"
end
[inline-code-end]

---