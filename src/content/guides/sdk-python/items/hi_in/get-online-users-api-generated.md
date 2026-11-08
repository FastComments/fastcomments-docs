किसी पेज के वर्तमान में ऑनलाइन दर्शक: वे लोग जिनका websocket सत्र इस समय पेज की सदस्यता लिए हुए है।
anonCount + totalCount लौटाता है (पूरे रूम के सदस्य, जिनमें वे अनाम दर्शक भी शामिल हैं जिन्हें हम सूचीबद्ध नहीं करते)।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक है जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ न छूटें। |

## प्रतिक्रिया

लौटाता है: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## उदाहरण

[inline-code-attrs-start title = 'get_online_users उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
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
    url_id = 'url_id_example' # str | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)।
    after_name = 'after_name_example' # str | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। (optional)
    after_user_id = 'after_user_id_example' # str | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक है जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ न छूटें। (optional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]