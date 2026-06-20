## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| notificationId | string | path | כן |  |
| newStatus | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_user_notification_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
notification_id = 'notification_id_example' # מחרוזת | 
new_status = 'read' # מחרוזת | 
opts = {
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.update_user_notification_status(tenant_id, notification_id, new_status, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_status: #{e}"
end
[inline-code-end]

---