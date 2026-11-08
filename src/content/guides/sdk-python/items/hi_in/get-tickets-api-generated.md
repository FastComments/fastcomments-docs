## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| state | number | query | नहीं |  |
| skip | number | query | नहीं |  |
| limit | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets_response.py)

## उदाहरण

[inline-code-attrs-start title = 'get_tickets उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTicketsOptions
from client.models.get_tickets_response import GetTicketsResponse
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और इसका डिफ़ॉल्ट https://fastcomments.com है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# क्लाइंट को API सर्वर की सुरक्षा नीति के अनुसार
# प्रमाणीकरण और ऑथराइज़ेशन पैरामीटर कॉन्फ़िगर करने होंगे।
# प्रत्येक ऑथ मेथड के उदाहरण नीचे दिए गए हैं, अपने ऑथ यूज़ केस के
# अनुरूप उदाहरण का उपयोग करें।

# API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएँ
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (वैकल्पिक)
    state = 3.4 # float |  (वैकल्पिक)
    skip = 3.4 # float |  (वैकल्पिक)
    limit = 3.4 # float |  (वैकल्पिक)

    try:
        api_response = api_instance.get_tickets(tenant_id, GetTicketsOptions(user_id=user_id, state=state, skip=skip, limit=limit))
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]