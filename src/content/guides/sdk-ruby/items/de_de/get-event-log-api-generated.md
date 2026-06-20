req
tenantId
urlId
userIdWS

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| userIdWS | string | query | Ja |  |
| startTime | integer | query | Ja |  |
| endTime | integer | query | Nein |  |

## Antwort

Gibt zurück: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_event_log_response.rb)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_event_log'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
user_id_ws = 'user_id_ws_example' # String | 
start_time = 789 # Integer | 
opts = {
  end_time: 789 # Integer | 
}

begin
  
  result = api_instance.get_event_log(tenant_id, url_id, user_id_ws, start_time, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_event_log: #{e}"
end
[inline-code-end]

---