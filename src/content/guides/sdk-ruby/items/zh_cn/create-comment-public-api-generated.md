---
## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| urlId | string | 查询 | 是 |  |
| broadcastId | string | 查询 | 是 |  |
| sessionId | string | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_comment_public200_response.rb)

## 示例

[inline-code-attrs-start title = 'create_comment_public 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
url_id = 'url_id_example' # 字符串 | 
broadcast_id = 'broadcast_id_example' # 字符串 | 
comment_data = FastCommentsClient::CommentData.new({commenter_name: 'commenter_name_example', comment: 'comment_example', url: 'url_example', url_id: 'url_id_example'}) # CommentData | 
opts = {
  session_id: 'session_id_example', # 字符串 | 
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.create_comment_public(tenant_id, url_id, broadcast_id, comment_data, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->create_comment_public: #{e}"
end
[inline-code-end]

---