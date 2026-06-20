---
Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

ドキュメントをグループ化（groupBy が提供されている場合）し、複数の操作を適用して集計します。sum、countDistinct、avg などのさまざまな操作がサポートされています。

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| parentTenantId | string | query | いいえ |  |
| includeStats | boolean | query | いいえ |  |

## Response

Returns: [`AggregateResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/aggregate_response.rb)

戻り値: [`AggregateResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/aggregate_response.rb)

## Example

[inline-code-attrs-start title = 'aggregate の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーのプレフィックスを設定するには、次の行のコメントを解除してください。例: 'Bearer'（デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
aggregation_request = FastCommentsClient::AggregationRequest.new({resource_name: 'resource_name_example', operations: [FastCommentsClient::AggregationOperation.new({field: 'field_example', op: FastCommentsClient::AggregationOpType::SUM})]}) # AggregationRequest | 
opts = {
  parent_tenant_id: 'parent_tenant_id_example', # String | 
  include_stats: true # Boolean | 
}

begin
  
  result = api_instance.aggregate(tenant_id, aggregation_request, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->aggregate: #{e}"
end
[inline-code-end]

---