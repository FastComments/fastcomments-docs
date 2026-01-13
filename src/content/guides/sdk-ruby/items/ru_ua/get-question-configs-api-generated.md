## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Ответ

Возвращает: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_question_configs200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_question_configs'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка авторизации
FastCommentsClient.configure do |config|
  # Настройте авторизацию по API-ключу: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Раскомментируйте следующую строку, чтобы задать префикс для API-ключа, e.g. 'Bearer' (defaults to nil)
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