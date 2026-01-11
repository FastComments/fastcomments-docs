## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| notificationId | string | path | Da |  |
| newStatus | string | path | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status200_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer update_user_notification_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Niz | 
notification_id = 'notification_id_example' # Niz | 
new_status = 'read' # Niz | 
opts = {
  sso: 'sso_example' # Niz | 
}

begin
  
  result = api_instance.update_user_notification_status(tenant_id, notification_id, new_status, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_status: #{e}"
end
[inline-code-end]

---