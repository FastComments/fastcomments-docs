## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| notificationId | string | path | Oui |  |
| newStatus | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status200_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de update_user_notification_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
notification_id = 'notification_id_example' # Chaîne | 
new_status = 'read' # Chaîne | 
opts = {
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.update_user_notification_status(tenant_id, notification_id, new_status, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_status: #{e}"
end
[inline-code-end]