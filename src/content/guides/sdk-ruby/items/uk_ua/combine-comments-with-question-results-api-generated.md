## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| questionId | string | query | Ні |  |
| questionIds | array | query | Ні |  |
| urlId | string | query | Ні |  |
| startDate | string | query | Ні |  |
| forceRecalculate | boolean | query | Ні |  |
| minValue | number | query | Ні |  |
| maxValue | number | query | Ні |  |
| limit | number | query | Ні |  |

## Відповідь

Повертає: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/combine_question_results_with_comments_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад combine_comments_with_question_results'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# налаштування авторизації
FastCommentsClient.configure do |config|
  # Налаштуйте авторизацію за допомогою API-ключа: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Розкоментуйте наведений рядок, щоб встановити префікс для API-ключа, напр., 'Bearer' (за замовчуванням nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  question_id: 'question_id_example', # String | 
  question_ids: ['inner_example'], # Array<String> | 
  url_id: 'url_id_example', # String | 
  start_date: Time.parse('2013-10-20T19:20:30+01:00'), # Time | 
  force_recalculate: true, # Boolean | 
  min_value: 1.2, # Float | 
  max_value: 1.2, # Float | 
  limit: 1.2 # Float | 
}

begin
  
  result = api_instance.combine_comments_with_question_results(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->combine_comments_with_question_results: #{e}"
end
[inline-code-end]

---