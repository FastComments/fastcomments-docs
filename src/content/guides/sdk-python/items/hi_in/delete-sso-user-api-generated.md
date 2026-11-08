## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| deleteComments | boolean | query | नहीं |  |
| commentDeleteMode | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## उदाहरण

[inline-code-attrs-start title = 'delete_sso_user उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import DeleteSsoUserOptions
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# क्लाइंट को API सर्वर की सुरक्षा नीति के अनुसार
# प्रमाणीकरण और ऑथराइज़ेशन पैरामीटर कॉन्फ़िगर करने होंगे।
# प्रत्येक ऑथ मेथड के उदाहरण नीचे दिए गए हैं, अपने ऑथ उपयोग के मामले को
# पूरा करने वाले उदाहरण का उपयोग करें।

# API कुंजी ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# यदि आवश्यक हो तो API कुंजी के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = True # bool |  (वैकल्पिक)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, DeleteSsoUserOptions(delete_comments=delete_comments, comment_delete_mode=comment_delete_mode))
        print("The response of DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]