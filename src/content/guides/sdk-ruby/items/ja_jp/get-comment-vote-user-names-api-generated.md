## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| commentId | string | path | はい |  |
| dir | integer | query | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_vote_user_names_success_response.rb)

## 例

[inline-code-attrs-start title = 'get_comment_vote_user_names の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 文字列 | 
comment_id = 'comment_id_example' # 文字列 | 
dir = 56 # 整数 | 
opts = {
  sso: 'sso_example' # 文字列 | 
}

begin
  
  result = api_instance.get_comment_vote_user_names(tenant_id, comment_id, dir, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comment_vote_user_names: #{e}"
end
[inline-code-end]