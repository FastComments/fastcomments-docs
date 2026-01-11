## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| commentId | string | 路径 | 是 |  |
| broadcastId | string | 查询 | 是 |  |
| editKey | string | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回：[`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_comment_public200_response.rb)

## 示例

[inline-code-attrs-start title = 'delete_comment_public 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
comment_id = 'comment_id_example' # 字符串 | 
broadcast_id = 'broadcast_id_example' # 字符串 | 
opts = {
  edit_key: 'edit_key_example', # 字符串 | 
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.delete_comment_public(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->delete_comment_public: #{e}"
end
[inline-code-end]

---