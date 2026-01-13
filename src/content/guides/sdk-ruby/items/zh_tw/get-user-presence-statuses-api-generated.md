---
## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlIdWS | string | query | 是 |  |
| userIds | string | query | 是 |  |

## 回應

回傳：[`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_presence_statuses200_response.rb)

## 範例

[inline-code-attrs-start title = 'get_user_presence_statuses 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字串 | 
url_id_ws = 'url_id_ws_example' # 字串 | 
user_ids = 'user_ids_example' # 字串 | 

begin
  
  result = api_instance.get_user_presence_statuses(tenant_id, url_id_ws, user_ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_presence_statuses: #{e}"
end
[inline-code-end]

---