## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/unblock_success.py)

## उदाहरण

[inline-code-attrs-start title = 'un_block_user_from_comment उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import UnBlockUserFromCommentOptions
from client.models.un_block_from_comment_params import UnBlockFromCommentParams
from client.models.unblock_success import UnblockSuccess
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# क्लाइंट को API सर्वर की सुरक्षा नीति के अनुसार
# प्रमाणीकरण और ऑथराइज़ेशन पैरामीटर कॉन्फ़िगर करना होगा।
# प्रत्येक ऑथ मेथड के उदाहरण नीचे दिए गए हैं, अपने ऑथ यूज़ केस के
# अनुरूप उदाहरण का उपयोग करें।

# API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    un_block_from_comment_params = client.UnBlockFromCommentParams() # UnBlockFromCommentParams | 
    user_id = 'user_id_example' # str |  (वैकल्पिक)
    anon_user_id = 'anon_user_id_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.un_block_user_from_comment(tenant_id, id, un_block_from_comment_params, UnBlockUserFromCommentOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("The response of DefaultApi->un_block_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->un_block_user_from_comment: %s\n" % e)
[inline-code-end]