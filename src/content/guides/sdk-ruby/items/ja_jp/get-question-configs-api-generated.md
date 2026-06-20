## パラメータ

| 名前 | 型 | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_question_configs_response.rb)

## 例

[inline-code-attrs-start title = 'get_question_configs の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # API キー認証を設定する: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーにプレフィックスを設定するには、次の行のコメントアウトを解除してください（例: 'Bearer'、デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_question_configs(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_question_configs: #{e}"
end
[inline-code-end]