## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| text-search | string | query | नहीं |  |
| byIPFromComment | string | query | नहीं |  |
| filter | string | query | नहीं |  |
| searchFilters | string | query | नहीं |  |
| demo | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_count_comments_response.py)

## उदाहरण

[inline-code-attrs-start title = 'get_count उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetCountOptions
from client.models.moderation_api_count_comments_response import ModerationAPICountCommentsResponse
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
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    text_search = 'text_search_example' # str |  (वैकल्पिक)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (वैकल्पिक)
    filter = 'filter_example' # str |  (वैकल्पिक)
    search_filters = 'search_filters_example' # str |  (वैकल्पिक)
    demo = True # bool |  (वैकल्पिक)
    sso = 'sso_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.get_count(tenant_id, GetCountOptions(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filter=filter, search_filters=search_filters, demo=demo, sso=sso))
        print("The response of ModerationApi->get_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_count: %s\n" % e)
[inline-code-end]