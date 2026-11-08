## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| notificationId | string | path | हाँ |  |
| newStatus | string | path | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `UpdateUserNotificationStatusResponse`

## उदाहरण

[inline-code-attrs-start title = 'update_user_notification_status उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
notification_id = 'notification_id_example' # String | 
new_status = 'read' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.update_user_notification_status(tenant_id, notification_id, new_status, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_status: #{e}"
end
[inline-code-end]