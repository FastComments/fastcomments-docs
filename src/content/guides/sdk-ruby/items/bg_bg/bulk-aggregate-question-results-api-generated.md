## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| forceRecalculate | boolean | query | Не |  |

## Отговор

Връща: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/bulk_aggregate_question_results200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за bulk_aggregate_question_results'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка на удостоверяване
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Премахнете коментара от следния ред, за да зададете префикс за API ключа, например 'Bearer' (по подразбиране nil)
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