## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| fromName | string | query | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад send_invite'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# налаштування авторизації
FastCommentsClient.configure do |config|
  # Налаштуйте авторизацію за допомогою API-ключа: api_key
  # Розкоментуйте наступний рядок, щоб встановити префікс для API-ключа, наприклад 'Bearer' (за замовчуванням nil)
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
from_name = 'from_name_example' # String | 

begin
  
  result = api_instance.send_invite(tenant_id, id, from_name)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->send_invite: #{e}"
end
[inline-code-end]

---