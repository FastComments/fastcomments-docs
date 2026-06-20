## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| commentId | string | 路径 | 是 |  |
| broadcastId | string | 查询 | 是 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/change_comment_pin_status_response.rb)

## 示例

[inline-code-attrs-start title = 'un_pin_comment 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
  
  result = api_instance.un_pin_comment(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->un_pin_comment: #{e}"
end
[inline-code-end]