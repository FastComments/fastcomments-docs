## Параметры

| Имя | Тип | Местоположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Ответ

Возвращает: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_success_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример delete_user_badge'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка авторизации
FastCommentsClient.configure do |config|
  # Настройка авторизации ключом API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Раскомментируйте следующую строку, чтобы задать префикс для ключа API, например 'Bearer' (по умолчанию nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.delete_user_badge(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_user_badge: #{e}"
end
[inline-code-end]

---