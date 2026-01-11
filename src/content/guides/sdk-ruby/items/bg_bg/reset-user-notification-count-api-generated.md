## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/reset_user_notifications200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за reset_user_notification_count'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Низ | 
opts = {
  sso: 'sso_example' # Низ | 
}

begin
  
  result = api_instance.reset_user_notification_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->reset_user_notification_count: #{e}"
end
[inline-code-end]