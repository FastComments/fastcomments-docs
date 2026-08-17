## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_cached_notification_count_response.rb)

## Пример

[inline-code-attrs-start title = 'Primer get_cached_notification_count'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# подеси ауторизацију
FastCommentsClient.configure do |config|
  # Конфигуриши ауторизацију API кључа: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Одкоментариши следећи ред да подесиш префикс за API кључ, нпр. 'Bearer' (подразумевано је nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # Ниска | 
id = 'id_example' # Ниска | 

begin
  
  result = api_instance.get_cached_notification_count(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_cached_notification_count: #{e}"
end
[inline-code-end]