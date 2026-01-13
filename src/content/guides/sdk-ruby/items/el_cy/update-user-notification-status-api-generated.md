## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| notificationId | string | path | Ναι |  |
| newStatus | string | path | Ναι |  |
| sso | string | query | Όχι |  |

## Response

Επιστρέφει: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status200_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα update_user_notification_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Συμβολοσειρά | 
notification_id = 'notification_id_example' # Συμβολοσειρά | 
new_status = 'read' # Συμβολοσειρά | 
opts = {
  sso: 'sso_example' # Συμβολοσειρά | 
}

begin
  
  result = api_instance.update_user_notification_status(tenant_id, notification_id, new_status, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_status: #{e}"
end
[inline-code-end]

---