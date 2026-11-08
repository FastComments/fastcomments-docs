req
tenantId
urlId
userIdWS

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| userIdWS | string | query | हाँ |  |
| startTime | integer | query | हाँ |  |
| endTime | integer | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_event_log_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_global_event_log उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
  
  result = api_instance.get_global_event_log(tenant_id, url_id, user_id_ws, start_time, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_global_event_log: #{e}"
end
[inline-code-end]