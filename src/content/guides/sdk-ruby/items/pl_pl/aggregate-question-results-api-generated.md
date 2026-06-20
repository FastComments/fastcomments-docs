## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## Odpowiedź

Zwraca: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/aggregate_question_results_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład aggregate_question_results'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ustaw autoryzację
FastCommentsClient.configure do |config|
  # Skonfiguruj autoryzację klucza API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentuj następującą linię, aby ustawić prefiks dla klucza API, np. 'Bearer' (domyślnie nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  question_id: 'question_id_example', # String | 
  question_ids: ['inner_example'], # Array<String> | 
  url_id: 'url_id_example', # String | 
  time_bucket: FastCommentsClient::AggregateTimeBucket::DAY, # AggregateTimeBucket | 
  start_date: Time.parse('2013-10-20T19:20:30+01:00'), # Time | 
  force_recalculate: true # Boolean | 
}

begin
  
  result = api_instance.aggregate_question_results(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->aggregate_question_results: #{e}"
end
[inline-code-end]