req
tenantId
urlId
userIdWS

## 參數

| 名稱 | 類型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路徑 | 是 |  |
| urlId | string | 查詢 | 是 |  |
| userIdWS | string | 查詢 | 是 |  |
| startTime | integer | 查詢 | 是 |  |
| endTime | integer | 查詢 | 否 |  |

## 回應

回傳：[`GetEventLogResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_event_log_response.rb)

## 範例

[inline-code-attrs-start title = 'get_global_event_log 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字串 | 
url_id = 'url_id_example' # 字串 | 
user_id_ws = 'user_id_ws_example' # 字串 | 
start_time = 789 # 整數 | 
opts = {
  end_time: 789 # 整數 | 
}

begin
  
  result = api_instance.get_global_event_log(tenant_id, url_id, user_id_ws, start_time, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_global_event_log: #{e}"
end
[inline-code-end]

---