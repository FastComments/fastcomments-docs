## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Отговор

Връща: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_user_badge200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за create_user_badge'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка на удостоверяване
FastCommentsClient.configure do |config|
  # Конфигуриране на удостоверяване с API ключ: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Разкоментирайте следния ред, за да зададете префикс за API ключа, напр. 'Bearer' (по подразбиране nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_user_badge_params = FastCommentsClient::CreateUserBadgeParams.new({user_id: 'user_id_example', badge_id: 'badge_id_example'}) # CreateUserBadgeParams | 

begin
  
  result = api_instance.create_user_badge(tenant_id, create_user_badge_params)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_user_badge: #{e}"
end
[inline-code-end]