req
tenantId
urlId
userIdWS

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| userIdWS | string | query | Da |  |
| startTime | integer | query | Da |  |
| endTime | integer | query | Da |  |

## Odgovor

Vraća: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_event_log200_response.rb)

## Primjer

[inline-code-attrs-start title = 'get_global_event_log Primjer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Niz | 
url_id = 'url_id_example' # Niz | 
user_id_ws = 'user_id_ws_example' # Niz | 
start_time = 789 # Cijeli broj | 
end_time = 789 # Cijeli broj | 

begin
  
  result = api_instance.get_global_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_global_event_log: #{e}"
end
[inline-code-end]