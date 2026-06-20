req
tenantId
urlId
userIdWS

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| userIdWS | string | query | Sì |  |
| startTime | integer | query | Sì |  |
| endTime | integer | query | No |  |

## Risposta

Restituisce: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_event_log_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio get_event_log'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Stringa | 
url_id = 'url_id_example' # Stringa | 
user_id_ws = 'user_id_ws_example' # Stringa | 
start_time = 789 # Intero | 
opts = {
  end_time: 789 # Intero | 
}

begin
  
  result = api_instance.get_event_log(tenant_id, url_id, user_id_ws, start_time, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_event_log: #{e}"
end
[inline-code-end]