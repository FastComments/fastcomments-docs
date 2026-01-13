## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_email_template200_response.rb)

## Пример

[inline-code-attrs-start title = 'get_email_template Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка на авторизация
FastCommentsClient.configure do |config|
  # Конфигуриране на авторизация чрез API ключ: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Разкоментирайте следния ред, за да зададете префикс за API ключа, напр. 'Bearer' (по подразбиране nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_email_template(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_email_template: #{e}"
end
[inline-code-end]

---