किसी टेनेंट के लिए बल्क यूज़र जानकारी। दिए गए userIds के लिए, User / SSOUser से डिस्प्ले जानकारी लौटाता है।
कमेंट विजेट द्वारा उपयोग किया जाता है जो presence इवेंट के माध्यम से अभी दिखाई दिए यूज़र्स को समृद्ध (enrich) करने के लिए है।
कोई पेज संदर्भ नहीं: गोपनीयता समान रूप से लागू की जाती है (प्राइवेट प्रोफाइल मास्क कर दिए जाते हैं)।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| ids | string | query | हाँ | कॉमा से अलग किए गए userIds। |

## प्रतिक्रिया

लौटाता है: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## उदाहरण

[inline-code-attrs-start title = 'get_users_info उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
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
    ids = 'ids_example' # str | Comma-delimited userIds.

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]