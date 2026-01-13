## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlIdWS | string | query | כן |  |
| userIds | string | query | כן |  |

## תגובה

מחזיר: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_presence_statuses200_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_presence_statuses'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
url_id_ws = 'url_id_ws_example' # מחרוזת | 
user_ids = 'user_ids_example' # מחרוזת | 

begin
  
  result = api_instance.get_user_presence_statuses(tenant_id, url_id_ws, user_ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_presence_statuses: #{e}"
end
[inline-code-end]

---