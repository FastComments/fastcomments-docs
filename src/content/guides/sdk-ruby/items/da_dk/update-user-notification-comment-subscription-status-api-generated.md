Aktivér eller deaktiver notifikationer for en specifik kommentar.

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| notificationId | string | path | Ja |  |
| optedInOrOut | string | path | Ja |  |
| commentId | string | query | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status200_response.rb)

## Eksempel

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Streng | 
notification_id = 'notification_id_example' # Streng | 
opted_in_or_out = 'in' # Streng | 
comment_id = 'comment_id_example' # Streng | 
opts = {
  sso: 'sso_example' # Streng | 
}

begin
  
  result = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_comment_subscription_status: #{e}"
end
[inline-code-end]