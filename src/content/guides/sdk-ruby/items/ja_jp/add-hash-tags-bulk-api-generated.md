## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | いいえ |  |

## レスポンス

戻り値: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/bulk_create_hash_tags_response.rb)

## 例

[inline-code-attrs-start title = 'add_hash_tags_bulk の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーにプレフィックスを設定するには、以下の行のコメントを解除してください。例: 'Bearer'（デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
opts = {
  tenant_id: 'tenant_id_example', # String | 
  bulk_create_hash_tags_body: FastCommentsClient::BulkCreateHashTagsBody.new({tags: [FastCommentsClient::BulkCreateHashTagsBodyTagsInner.new({tag: 'tag_example'})]}) # BulkCreateHashTagsBody | 
}

begin
  
  result = api_instance.add_hash_tags_bulk(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_hash_tags_bulk: #{e}"
end
[inline-code-end]