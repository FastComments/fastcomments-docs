किसी विशिष्ट कमेंट के लिए नोटिफिकेशन सक्षम या अक्षम करें।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| notificationId | string | path | हाँ |  |
| optedInOrOut | string | path | हाँ |  |
| commentId | string | query | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_comment_subscription_status_response.py)

## उदाहरण

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_comment_subscription_status_response import UpdateUserNotificationCommentSubscriptionStatusResponse
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के एक इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    notification_id = 'notification_id_example' # str | 
    opted_in_or_out = 'opted_in_or_out_example' # str | 
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, sso=sso)
        print("The response of PublicApi->update_user_notification_comment_subscription_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_comment_subscription_status: %s\n" % e)
[inline-code-end]