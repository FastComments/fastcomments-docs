必填
tenantId
urlId
userIdWS

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| userIdWS | string | query | 是 |  |
| startTime | integer | query | 是 |  |
| endTime | integer | query | 是 |  |

## 响应

返回: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_event_log200_response.rb)

## 示例

[inline-code-attrs-start title = 'get_event_log 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
url_id = 'url_id_example' # 字符串 | 
user_id_ws = 'user_id_ws_example' # 字符串 | 
start_time = 789 # 整数 | 
end_time = 789 # 整数 | 

begin
  
  result = api_instance.get_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_event_log: #{e}"
end
[inline-code-end]

---