req
tenantId
urlId
userIdWS

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| userIdWS | string | query | Evet |  |
| startTime | integer | query | Evet |  |
| endTime | integer | query | Hayır |  |

## Yanıt

Döndürür: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_event_log_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_global_event_log Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Dize | 
url_id = 'url_id_example' # Dize | 
user_id_ws = 'user_id_ws_example' # Dize | 
start_time = 789 # Tamsayı | 
opts = {
  end_time: 789 # Tamsayı | 
}

begin
  
  result = api_instance.get_global_event_log(tenant_id, url_id, user_id_ws, start_time, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_global_event_log: #{e}"
end
[inline-code-end]