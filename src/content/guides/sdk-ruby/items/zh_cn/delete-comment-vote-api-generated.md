---
## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| commentId | string | 路径 | 是 |  |
| voteId | string | 路径 | 是 |  |
| urlId | string | 查询 | 是 |  |
| broadcastId | string | 查询 | 是 |  |
| editKey | string | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_comment_vote200_response.rb)

## 示例

[inline-code-attrs-start title = 'delete_comment_vote 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
comment_id = 'comment_id_example' # 字符串 | 
vote_id = 'vote_id_example' # 字符串 | 
url_id = 'url_id_example' # 字符串 | 
broadcast_id = 'broadcast_id_example' # 字符串 | 
opts = {
  edit_key: 'edit_key_example', # 字符串 | 
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->delete_comment_vote: #{e}"
end
[inline-code-end]

---