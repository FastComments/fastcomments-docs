किसी पेज के लिए नोटिफिकेशन सक्षम या अक्षम करें। जब उपयोगकर्ता किसी पेज की सदस्यता लेते हैं, तो नई रूट कमेंट्स के लिए नोटिफिकेशन बनाए जाते हैं, और साथ ही

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | हाँ |  |
| url | string | query | हाँ |  |
| pageTitle | string | query | हाँ |  |
| subscribedOrUnsubscribed | string | path | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_page_subscription_status_response.py)

## उदाहरण

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_page_subscription_status_response import UpdateUserNotificationPageSubscriptionStatusResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    url = 'url_example' # str | 
    page_title = 'page_title_example' # str | 
    subscribed_or_unsubscribed = 'subscribed_or_unsubscribed_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, sso=sso)
        print("The response of PublicApi->update_user_notification_page_subscription_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_page_subscription_status: %s\n" % e)
[inline-code-end]