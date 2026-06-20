---
## Параметри

| Назив | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Пример

[inline-code-attrs-start title = 'delete_email_template Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Подешавање ауторизације
FastCommentsClient.configure do |config|
  # Конфигуришите аутентификацију помоћу API кључа: api_key
  # Ако је потребно, откоментаришите следећи ред да бисте поставили префикс за API кључ, нпр. 'Bearer' (подразумевано nil)
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.delete_email_template(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_email_template: #{e}"
end
[inline-code-end]

---