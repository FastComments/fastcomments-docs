## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |
| sendEmail | string | query | いいえ |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## 例

[inline-code-attrs-start title = 'delete_moderator の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証を設定
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーに接頭辞（例: 'Bearer'）を設定するには、以下の行のコメントを外してください（デフォルトはnil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  send_email: 'send_email_example' # String | 
}

begin
  
  result = api_instance.delete_moderator(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_moderator: #{e}"
end
[inline-code-end]

---