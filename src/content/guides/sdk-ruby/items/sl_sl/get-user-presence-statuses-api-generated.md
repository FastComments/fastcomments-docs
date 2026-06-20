## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlIdWS | string | query | Da |  |
| userIds | string | query | Da |  |

## Odgovor

Vrne: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_presence_statuses_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer get_user_presence_statuses'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
  puts "Napaka pri klicu PublicApi->get_user_presence_statuses: #{e}"
end
[inline-code-end]

---