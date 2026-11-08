## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | नहीं | यह निर्धारित करने के लिए उपयोग किया जाता है कि क्या वर्तमान पृष्ठ सब्सक्राइब है। |
| pageSize | integer | query | नहीं |  |
| afterId | string | query | नहीं |  |
| includeContext | boolean | query | नहीं |  |
| afterCreatedAt | integer | query | नहीं |  |
| unreadOnly | boolean | query | नहीं |  |
| dmOnly | boolean | query | नहीं |  |
| noDm | boolean | query | नहीं |  |
| includeTranslations | boolean | query | नहीं |  |
| includeTenantNotifications | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## उदाहरण

[inline-code-attrs-start title = 'get_user_notifications उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetUserNotificationsOptions
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और इसका डिफ़ॉल्ट https://fastcomments.com है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के एक इंस्टेंस के साथ कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | यह निर्धारित करने के लिए उपयोग किया जाता है कि क्या वर्तमान पृष्ठ सब्सक्राइब है। (optional)
    page_size = 56 # int |  (optional)
    after_id = 'after_id_example' # str |  (optional)
    include_context = True # bool |  (optional)
    after_created_at = 56 # int |  (optional)
    unread_only = True # bool |  (optional)
    dm_only = True # bool |  (optional)
    no_dm = True # bool |  (optional)
    include_translations = True # bool |  (optional)
    include_tenant_notifications = True # bool |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, GetUserNotificationsOptions(url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso))
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]