---
## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| id | string | query | כן |  |

## תגובה

מחזיר: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_v2_page_react_users_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_v2_page_react_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
url_id = 'url_id_example' # מחרוזת | 
id = 'id_example' # מחרוזת | 

begin
  
  result = api_instance.get_v2_page_react_users(tenant_id, url_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_v2_page_react_users: #{e}"
end
[inline-code-end]

---