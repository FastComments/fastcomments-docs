---
## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| urlId | string | query | 否 |  |
| fromCommentId | string | query | 否 |  |
| viewed | boolean | query | 否 |  |
| type | string | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回：[`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_notifications_response.rb)

## 示例

[inline-code-attrs-start title = 'get_notifications 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 设置授权
FastCommentsClient.configure do |config|
  # 配置 API 密钥授权：api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消注释下面一行以为 API 密钥设置前缀，例如 'Bearer'（默认值为 nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
opts = {
  user_id: 'user_id_example', # 字符串 | 
  url_id: 'url_id_example', # 字符串 | 
  from_comment_id: 'from_comment_id_example', # 字符串 | 
  viewed: true, # 布尔值 | 
  type: 'type_example', # 字符串 | 
  skip: 1.2 # 浮点数 | 
}

begin
  
  result = api_instance.get_notifications(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_notifications: #{e}"
end
[inline-code-end]

---