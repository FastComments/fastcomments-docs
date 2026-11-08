किसी विशिष्ट कमेंट के लिए नोटिफिकेशन चालू या बंद करें।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| notificationId | string | path | हाँ |  |
| optedInOrOut | string | path | हाँ |  |
| commentId | string | query | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `UpdateUserNotificationCommentSubscriptionStatusResponse`

## उदाहरण

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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