## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | упит | Да |  |
| id | string | путања | Да |  |

## Одговор

Враћа: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_badge200_response.rb)

## Примјер

[inline-code-attrs-start title = 'get_user_badge Примјер'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# подесити ауторизацију
FastCommentsClient.configure do |config|
  # Подесити ауторизацију помоћу API кључа: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Откоментирајте следећу линију да бисте поставили префикс за API кључ, нпр. 'Bearer' (подразумевано nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_user_badge(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user_badge: #{e}"
end
[inline-code-end]

---