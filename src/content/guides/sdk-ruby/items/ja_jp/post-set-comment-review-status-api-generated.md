## パラメータ

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| reviewed | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## レスポンス

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## 例

[inline-code-attrs-start title = 'post_set_comment_review_status 例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # 文字列 | 
comment_id = 'comment_id_example' # 文字列 | 
opts = {
  reviewed: true, # 真偽値 | 
  broadcast_id: 'broadcast_id_example', # 文字列 | 
  sso: 'sso_example' # 文字列 | 
}

begin
  
  result = api_instance.post_set_comment_review_status(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_set_comment_review_status: #{e}"
end
[inline-code-end]