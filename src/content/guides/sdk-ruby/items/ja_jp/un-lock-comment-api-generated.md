---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | パス | はい |  |
| commentId | string | パス | はい |  |
| broadcastId | string | クエリ | はい |  |
| sso | string | クエリ | いいえ |  |

## レスポンス

戻り値: [`LockComment200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/lock_comment200_response.rb)

## 例

[inline-code-attrs-start title = 'un_lock_comment の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
  
  result = api_instance.un_lock_comment(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->un_lock_comment: #{e}"
end
[inline-code-end]

---