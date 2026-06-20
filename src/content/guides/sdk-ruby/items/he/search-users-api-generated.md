## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| usernameStartsWith | string | query | לא |  |
| mentionGroupIds | array | query | לא |  |
| sso | string | query | לא |  |
| searchSection | string | query | לא |  |

## תגובה

מחזיר: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/search_users_result.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-search_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
url_id = 'url_id_example' # מחרוזת | 
opts = {
  username_starts_with: 'username_starts_with_example', # מחרוזת | 
  mention_group_ids: ['inner_example'], # מערך<מחרוזת> | 
  sso: 'sso_example', # מחרוזת | 
  search_section: 'fast' # מחרוזת | 
}

begin
  
  result = api_instance.search_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->search_users: #{e}"
end
[inline-code-end]

---