## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| commentId | string | path | はい |  |
| broadcastId | string | query | はい |  |
| editKey | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/set_comment_text200_response.rb)

## 例

[inline-code-attrs-start title = 'set_comment_text の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
broadcast_id = 'broadcast_id_example' # String | 
comment_text_update_request = FastCommentsClient::CommentTextUpdateRequest.new({comment: 'comment_example'}) # CommentTextUpdateRequest | 
opts = {
  edit_key: 'edit_key_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->set_comment_text: #{e}"
end
[inline-code-end]