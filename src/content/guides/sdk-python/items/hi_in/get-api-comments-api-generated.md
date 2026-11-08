## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| page | number | query | नहीं |  |
| count | number | query | नहीं |  |
| text-search | string | query | नहीं |  |
| byIPFromComment | string | query | नहीं |  |
| filters | string | query | नहीं |  |
| searchFilters | string | query | नहीं |  |
| sorts | string | query | नहीं |  |
| demo | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comments_response.py)

## उदाहरण

[inline-code-attrs-start title = 'get_api_comments उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetApiCommentsOptions
from client.models.moderation_api_get_comments_response import ModerationAPIGetCommentsResponse
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के एक इंस्टेंस के साथ कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 3.4 # float |  (वैकल्पिक)
    count = 3.4 # float |  (वैकल्पिक)
    text_search = 'text_search_example' # str |  (वैकल्पिक)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (वैकल्पिक)
    filters = 'filters_example' # str |  (वैकल्पिक)
    search_filters = 'search_filters_example' # str |  (वैकल्पिक)
    sorts = 'sorts_example' # str |  (वैकल्पिक)
    demo = True # bool |  (वैकल्पिक)
    sso = 'sso_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.get_api_comments(tenant_id, GetApiCommentsOptions(page=page, count=count, text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, demo=demo, sso=sso))
        print("ModerationApi->get_api_comments की प्रतिक्रिया:\n")
        pprint(api_response)
    except Exception as e:
        print("ModerationApi->get_api_comments को कॉल करते समय अपवाद: %s\n" % e)
[inline-code-end]