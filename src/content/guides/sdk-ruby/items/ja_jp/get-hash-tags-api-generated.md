## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| page | number | query | いいえ |  |

## レスポンス

返却値: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_hash_tags_response.rb)

## 例

[inline-code-attrs-start title = 'get_hash_tags の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーにプレフィックスを設定するには、次の行のコメントを解除してください。例: 'Bearer'（既定は nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  page: 1.2 # Float | 
}

begin
  
  result = api_instance.get_hash_tags(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_hash_tags: #{e}"
end
[inline-code-end]