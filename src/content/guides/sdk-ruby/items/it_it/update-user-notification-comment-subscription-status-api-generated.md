Abilita o disabilita le notifiche per un commento specifico.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| notificationId | string | path | Sì |  |
| optedInOrOut | string | path | Sì |  |
| commentId | string | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_comment_subscription_status_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di update_user_notification_comment_subscription_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Stringa | 
notification_id = 'notification_id_example' # Stringa | 
opted_in_or_out = 'in' # Stringa | 
comment_id = 'comment_id_example' # Stringa | 
opts = {
  sso: 'sso_example' # Stringa | 
}

begin
  
  result = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_comment_subscription_status: #{e}"
end
[inline-code-end]

---