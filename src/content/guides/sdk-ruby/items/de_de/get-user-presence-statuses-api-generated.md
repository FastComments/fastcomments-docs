## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlIdWS | string | query | Ja |  |
| userIds | string | query | Ja |  |

## Antwort

Gibt zurück: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_presence_statuses200_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_user_presence_statuses Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id_ws = 'url_id_ws_example' # String | 
user_ids = 'user_ids_example' # String | 

begin
  
  result = api_instance.get_user_presence_statuses(tenant_id, url_id_ws, user_ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_presence_statuses: #{e}"
end
[inline-code-end]

---