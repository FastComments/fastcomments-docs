Агрегира документе групишући их (ако је groupBy наведен) и примјењујући више операција.
Подржане су различите операције (нпр. sum, countDistinct, avg, итд.).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| parentTenantId | string | query | Не |  |
| includeStats | boolean | query | Не |  |

## Одговор

Враћа: [`AggregationResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/aggregation_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример агрегирања'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# подешавање ауторизације
FastCommentsClient.configure do |config|
  # Конфигуришите API кључ за ауторизацију: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Откоментујте сљедећу линију да бисте поставили префикс за API кључ, нпр. 'Bearer' (подразумјевано nil)
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