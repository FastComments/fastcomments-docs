## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| commentId | string | path | はい |  |
| broadcastId | string | query | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`LockComment200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/lock_comment200_response.rb)

## 例

[inline-code-attrs-start title = 'lock_comment の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 文字列 | 
comment_id = 'comment_id_example' # 文字列 | 
broadcast_id = 'broadcast_id_example' # 文字列 | 
opts = {
  sso: 'sso_example' # 文字列 | 
}

begin
  
  result = api_instance.lock_comment(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->lock_comment: #{e}"
end
[inline-code-end]