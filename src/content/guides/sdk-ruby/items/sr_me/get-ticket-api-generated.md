---
## Параметри

| Iме | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Враћа

Враћа: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_ticket_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_ticket'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Подешавање ауторизације
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Уклоните коментар са следеће линије да бисте поставили префикс за API кључ, нпр. 'Bearer' (подразумевано nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  user_id: 'user_id_example' # String | 
}

begin
  
  result = api_instance.get_ticket(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_ticket: #{e}"
end
[inline-code-end]

---