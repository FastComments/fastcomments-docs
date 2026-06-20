Aktiver eller deaktiver notifikationer for en bestemt kommentar.

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| notificationId | string | path | Ja |  |
| optedInOrOut | string | path | Ja |  |
| commentId | string | query | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_comment_subscription_status_response.rb)

## Eksempel

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
notification_id = 'notification_id_example' # String | 
opted_in_or_out = 'in' # String | 
comment_id = 'comment_id_example' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_comment_subscription_status: #{e}"
end
[inline-code-end]