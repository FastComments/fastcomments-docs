## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| postId | string | path | हाँ |  |
| isUndo | boolean | query | नहीं |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/react_feed_post_response.py)

## उदाहरण

[inline-code-attrs-start title = 'react_feed_post_public उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import ReactFeedPostPublicOptions
from client.models.react_body_params import ReactBodyParams
from client.models.react_feed_post_response import ReactFeedPostResponse
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और यह https://fastcomments.com पर डिफ़ॉल्ट होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    react_body_params = client.ReactBodyParams() # ReactBodyParams | 
    is_undo = True # bool |  (वैकल्पिक)
    broadcast_id = 'broadcast_id_example' # str |  (वैकल्पिक)
    sso = 'sso_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.react_feed_post_public(tenant_id, post_id, react_body_params, ReactFeedPostPublicOptions(is_undo=is_undo, broadcast_id=broadcast_id, sso=sso))
        print("The response of PublicApi->react_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->react_feed_post_public: %s\n" % e)
[inline-code-end]