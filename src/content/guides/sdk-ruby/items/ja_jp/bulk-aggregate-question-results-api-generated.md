## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| forceRecalculate | boolean | クエリ | いいえ |  |

## レスポンス

戻り値: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/bulk_aggregate_question_results200_response.rb)

## 例

[inline-code-attrs-start title = 'bulk_aggregate_question_results の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証を設定
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーにプレフィックスを設定するには次の行のコメントを解除してください。例: 'Bearer'（デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
bulk_aggregate_question_results_request = FastCommentsClient::BulkAggregateQuestionResultsRequest.new({aggregations: [FastCommentsClient::BulkAggregateQuestionItem.new({agg_id: 'agg_id_example'})]}) # BulkAggregateQuestionResultsRequest | 
opts = {
  force_recalculate: true # Boolean | 
}

begin
  
  result = api_instance.bulk_aggregate_question_results(tenant_id, bulk_aggregate_question_results_request, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->bulk_aggregate_question_results: #{e}"
end
[inline-code-end]

---