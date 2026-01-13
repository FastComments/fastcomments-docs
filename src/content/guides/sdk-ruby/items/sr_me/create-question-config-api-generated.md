## Параметри

| Назив | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_question_config200_response.rb)

## Пример

[inline-code-attrs-start title = 'create_question_config Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# подешавање овлашћења
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Откоментирајте следећу линију да бисте подесили префикс за API кључ, нпр. 'Bearer' (подразумевано nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_question_config_body = FastCommentsClient::CreateQuestionConfigBody.new({name: 'name_example', question: 'question_example', type: 'type_example', reporting_order: 3.56}) # CreateQuestionConfigBody | 

begin
  
  result = api_instance.create_question_config(tenant_id, create_question_config_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_question_config: #{e}"
end
[inline-code-end]

---