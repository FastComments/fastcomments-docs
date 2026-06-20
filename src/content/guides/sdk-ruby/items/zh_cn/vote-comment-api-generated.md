## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| commentId | string | 路径 | 是 |  |
| urlId | string | 查询 | 是 |  |
| broadcastId | string | 查询 | 是 |  |
| sessionId | string | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回: [`VoteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_response.rb)

## 示例

[inline-code-attrs-start title = 'vote_comment 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
url_id = 'url_id_example' # String | 
broadcast_id = 'broadcast_id_example' # String | 
vote_body_params = FastCommentsClient::VoteBodyParams.new({commenter_email: 'commenter_email_example', commenter_name: 'commenter_name_example', vote_dir: 'up', url: 'url_example'}) # VoteBodyParams | 
opts = {
  session_id: 'session_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.vote_comment(tenant_id, comment_id, url_id, broadcast_id, vote_body_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->vote_comment: #{e}"
end
[inline-code-end]

---