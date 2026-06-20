## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Пример

[inline-code-attrs-start title = 'update_moderator Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка на авторизация
FastCommentsClient.configure do |config|
  # конфигуриране на авторизация чрез API ключ: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # разкоментирайте следния ред, за да зададете префикс за API ключа, напр. 'Bearer' (по подразбиране nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_moderator_body = FastCommentsClient::UpdateModeratorBody.new # UpdateModeratorBody | 

begin
  
  result = api_instance.update_moderator(tenant_id, id, update_moderator_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_moderator: #{e}"
end
[inline-code-end]

---