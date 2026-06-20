Включить или отключить уведомления для конкретного комментария.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| notificationId | string | path | Да |  |
| optedInOrOut | string | path | Да |  |
| commentId | string | query | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_comment_subscription_status_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример update_user_notification_comment_subscription_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Строка | 
notification_id = 'notification_id_example' # Строка | 
opted_in_or_out = 'in' # Строка | 
comment_id = 'comment_id_example' # Строка | 
opts = {
  sso: 'sso_example' # Строка | 
}

begin
  
  result = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_comment_subscription_status: #{e}"
end
[inline-code-end]