## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| urlId | string | クエリ | いいえ |  |
| userId | string | クエリ | いいえ |  |
| startDate | string | クエリ | いいえ |  |
| questionId | string | クエリ | いいえ |  |
| questionIds | string | クエリ | いいえ |  |
| skip | number | クエリ | いいえ |  |

## レスポンス

戻り値: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_question_results_response.rb)

## 例

[inline-code-attrs-start title = 'get_question_results の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証のセットアップ
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーのプレフィックスを設定するには、次の行のコメントアウトを解除してください（例: 'Bearer'、デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  url_id: 'url_id_example', # String | 
  user_id: 'user_id_example', # String | 
  start_date: 'start_date_example', # String | 
  question_id: 'question_id_example', # String | 
  question_ids: 'question_ids_example', # String | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_question_results(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_question_results: #{e}"
end
[inline-code-end]