## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| pageSize | integer | query | Нет |  |
| afterId | string | query | Нет |  |
| includeContext | boolean | query | Нет |  |
| afterCreatedAt | integer | query | Нет |  |
| unreadOnly | boolean | query | Нет |  |
| dmOnly | boolean | query | Нет |  |
| noDm | boolean | query | Нет |  |
| includeTranslations | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_notifications200_response.rb)

## Пример

[inline-code-attrs-start title = 'get_user_notifications Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Строка | 
opts = {
  page_size: 56, # Целое число | 
  after_id: 'after_id_example', # Строка | 
  include_context: true, # Логическое значение | 
  after_created_at: 789, # Целое число | 
  unread_only: true, # Логическое значение | 
  dm_only: true, # Логическое значение | 
  no_dm: true, # Логическое значение | 
  include_translations: true, # Логическое значение | 
  sso: 'sso_example' # Строка | 
}

begin
  
  result = api_instance.get_user_notifications(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_notifications: #{e}"
end
[inline-code-end]

---