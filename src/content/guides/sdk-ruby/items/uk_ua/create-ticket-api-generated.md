## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Так |  |

## Відповідь

Повертає: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_ticket200_response.rb)

## Приклад

[inline-code-attrs-start title = 'create_ticket Приклад'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# налаштування авторизації
FastCommentsClient.configure do |config|
  # Налаштуйте авторизацію за допомогою API-ключа: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Розкоментуйте наступний рядок, щоб задати префікс для API-ключа, наприклад 'Bearer' (за замовчуванням nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 
create_ticket_body = FastCommentsClient::CreateTicketBody.new({subject: 'subject_example'}) # CreateTicketBody | 

begin
  
  result = api_instance.create_ticket(tenant_id, user_id, create_ticket_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_ticket: #{e}"
end
[inline-code-end]