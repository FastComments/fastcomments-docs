## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Нет |  |
| badgeId | string | query | Нет |  |
| type | number | query | Нет |  |
| displayedOnComments | boolean | query | Нет |  |
| limit | number | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_user_badges_response.rb)

## Пример

[inline-code-attrs-start title = 'get_user_badges Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка авторизации
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Раскомментируйте следующую строку, чтобы задать префикс для API-ключа, например 'Bearer' (по умолчанию nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  badge_id: 'badge_id_example', # String | 
  type: 1.2, # Float | 
  displayed_on_comments: true, # Boolean | 
  limit: 1.2, # Float | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_user_badges(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user_badges: #{e}"
end
[inline-code-end]

---