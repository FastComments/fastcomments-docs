req
tenantId
urlId
userIdWS

## Parametri

| Name | Tipo | Location | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| userIdWS | string | query | Sì |  |
| startTime | integer | query | Sì |  |
| endTime | integer | query | Sì |  |

## Risposta

Restituisce: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_event_log200_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_global_event_log'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Stringa | 
url_id = 'url_id_example' # Stringa | 
user_id_ws = 'user_id_ws_example' # Stringa | 
start_time = 789 # Intero | 
end_time = 789 # Intero | 

begin
  
  result = api_instance.get_global_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_global_event_log: #{e}"
end
[inline-code-end]