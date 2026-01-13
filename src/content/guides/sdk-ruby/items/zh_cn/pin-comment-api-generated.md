## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| commentId | string | path | 是 |  |
| broadcastId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`PinComment200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/pin_comment200_response.rb)

## 示例

[inline-code-attrs-start title = 'pin_comment 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
comment_id = 'comment_id_example' # 字符串 | 
broadcast_id = 'broadcast_id_example' # 字符串 | 
opts = {
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.pin_comment(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->pin_comment: #{e}"
end
[inline-code-end]

---