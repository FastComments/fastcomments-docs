## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| commentId | string | path | はい |  |
| broadcastId | string | query | はい |  |
| editKey | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/public_a_p_i_set_comment_text_response.rb)

## 例

[inline-code-attrs-start title = 'set_comment_text の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 文字列 | 
comment_id = 'comment_id_example' # 文字列 | 
broadcast_id = 'broadcast_id_example' # 文字列 | 
comment_text_update_request = FastCommentsClient::CommentTextUpdateRequest.new({comment: 'comment_example'}) # CommentTextUpdateRequest | 
opts = {
  edit_key: 'edit_key_example', # 文字列 | 
  sso: 'sso_example' # 文字列 | 
}

begin
  
  result = api_instance.set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->set_comment_text: #{e}"
end
[inline-code-end]

---