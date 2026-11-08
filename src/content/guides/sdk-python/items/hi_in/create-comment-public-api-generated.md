## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| broadcastId | string | query | हाँ |  |
| sessionId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comments_response_with_presence.py)

## उदाहरण

[inline-code-attrs-start title = 'create_comment_public उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import CreateCommentPublicOptions
from client.models.comment_data import CommentData
from client.models.save_comments_response_with_presence import SaveCommentsResponseWithPresence
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
    url_id = 'url_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    comment_data = client.CommentData() # CommentData | 
    session_id = 'session_id_example' # str |  (वैकल्पिक)
    sso = 'sso_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.create_comment_public(tenant_id, url_id, broadcast_id, comment_data, CreateCommentPublicOptions(session_id=session_id, sso=sso))
        print("The response of PublicApi->create_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_comment_public: %s\n" % e)
[inline-code-end]