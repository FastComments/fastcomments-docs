## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| usernameStartsWith | string | query | כן |  |
| mentionGroupIds | array | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/search_users200_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של search_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
url_id = 'url_id_example' # מחרוזת | 
username_starts_with = 'username_starts_with_example' # מחרוזת | 
opts = {
  mention_group_ids: ['inner_example'], # מערך<מחרוזת> | 
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.search_users(tenant_id, url_id, username_starts_with, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->search_users: #{e}"
end
[inline-code-end]

---