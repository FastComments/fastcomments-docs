## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| sure | string | query | No |  |

## レスポンス

返却: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## 例

[inline-code-attrs-start title = 'delete_tenant の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーにプレフィックス（例: 'Bearer'）を設定するには以下の行のコメントを外してください（デフォルトはnil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  sure: 'sure_example' # String | 
}

begin
  
  result = api_instance.delete_tenant(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_tenant: #{e}"
end
[inline-code-end]

---