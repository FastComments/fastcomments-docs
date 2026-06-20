## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | いいえ |  |

## レスポンス

戻り値: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_hash_tag_response.rb)

## 例

[inline-code-attrs-start title = 'add_hash_tag の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証のセットアップ
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーのプレフィックスを設定するには次の行のコメントを外してください。例: 'Bearer'（デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
opts = {
  tenant_id: 'tenant_id_example', # String | 
  create_hash_tag_body: FastCommentsClient::CreateHashTagBody.new({tag: 'tag_example'}) # CreateHashTagBody | 
}

begin
  
  result = api_instance.add_hash_tag(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_hash_tag: #{e}"
end
[inline-code-end]

---