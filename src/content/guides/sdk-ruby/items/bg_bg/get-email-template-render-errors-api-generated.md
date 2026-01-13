## Параметри

| Име | Type | Location | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_email_template_render_errors200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_email_template_render_errors'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка на авторизация
FastCommentsClient.configure do |config|
  # Конфигуриране на авторизация с API ключ: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Разкоментирайте следния ред, за да зададете префикс за API ключа, напр. 'Bearer' (по подразбиране nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # Низ | 
id = 'id_example' # Низ | 
opts = {
  skip: 1.2 # Число с плаваща запетая | 
}

begin
  
  result = api_instance.get_email_template_render_errors(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_email_template_render_errors: #{e}"
end
[inline-code-end]

---