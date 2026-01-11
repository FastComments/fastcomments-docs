req
tenantId
urlId
userIdWS

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| userIdWS | string | query | Да |  |
| startTime | integer | query | Да |  |
| endTime | integer | query | Да |  |

## Отговор

Връща: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_event_log200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за get_event_log'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Низ | 
url_id = 'url_id_example' # Низ | 
user_id_ws = 'user_id_ws_example' # Низ | 
start_time = 789 # Цяло число | 
end_time = 789 # Цяло число | 

begin
  
  result = api_instance.get_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_event_log: #{e}"
end
[inline-code-end]

---