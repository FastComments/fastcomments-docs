## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_create_user_badge_response.py)

## उदाहरण

[inline-code-attrs-start title = 'create_user_badge उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_create_user_badge_response import APICreateUserBadgeResponse
from client.models.create_user_badge_params import CreateUserBadgeParams
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# क्लाइंट को API सर्वर सुरक्षा नीति के अनुसार
# प्रमाणीकरण और अधिकृति पैरामीटर कॉन्फ़िगर करने होंगे।
# प्रत्येक प्रमाणीकरण विधि के उदाहरण नीचे दिए गए हैं, अपने
# प्रमाणीकरण उपयोग के मामले के अनुरूप उदाहरण का उपयोग करें।

# API कुंजी प्रमाणीकरण कॉन्फ़िगर करें: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# यदि आवश्यक हो तो API कुंजी के लिए उपसर्ग (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_user_badge_params = client.CreateUserBadgeParams() # CreateUserBadgeParams | 

    try:
        api_response = api_instance.create_user_badge(tenant_id, create_user_badge_params)
        print("The response of DefaultApi->create_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_user_badge: %s\n" % e)
[inline-code-end]