## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| afterId | string | query | नहीं |  |
| afterCreatedAt | integer | query | नहीं |  |
| unreadOnly | boolean | query | नहीं |  |
| dmOnly | boolean | query | नहीं |  |
| noDm | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications_response.py)

## उदाहरण

[inline-code-attrs-start title = 'reset_user_notifications उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import ResetUserNotificationsOptions
from client.models.reset_user_notifications_response import ResetUserNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (वैकल्पिक)
    after_created_at = 56 # int |  (वैकल्पिक)
    unread_only = True # bool |  (वैकल्पिक)
    dm_only = True # bool |  (वैकल्पिक)
    no_dm = True # bool |  (वैकल्पिक)
    sso = 'sso_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.reset_user_notifications(tenant_id, ResetUserNotificationsOptions(after_id=after_id, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, sso=sso))
        print("The response of PublicApi->reset_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->reset_user_notifications: %s\n" % e)
[inline-code-end]