## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`BlockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/block_success.py)

## उदाहरण

[inline-code-attrs-start title = 'block_user_from_comment उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import BlockUserFromCommentOptions
from client.models.block_from_comment_params import BlockFromCommentParams
from client.models.block_success import BlockSuccess
from client.rest import ApiException
from pprint import pprint

# होस्ट परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# क्लाइंट को API सर्वर सुरक्षा नीति के अनुसार
# प्रमाणीकरण और प्राधिकरण पैरामीटर कॉन्फ़िगर करने होंगे।
# प्रत्येक प्रमाणीकरण विधि के उदाहरण नीचे दिए गए हैं, अपने
# प्रमाणीकरण उपयोग के मामले के अनुरूप उदाहरण का उपयोग करें।

# API कुंजी प्राधिकरण कॉन्फ़िगर करें: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# यदि आवश्यक हो तो API कुंजी के लिए उपसर्ग (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API क्लाइंट के एक इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    block_from_comment_params = client.BlockFromCommentParams() # BlockFromCommentParams | 
    user_id = 'user_id_example' # str |  (वैकल्पिक)
    anon_user_id = 'anon_user_id_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.block_user_from_comment(tenant_id, id, block_from_comment_params, BlockUserFromCommentOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("The response of DefaultApi->block_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->block_user_from_comment: %s\n" % e)
[inline-code-end]