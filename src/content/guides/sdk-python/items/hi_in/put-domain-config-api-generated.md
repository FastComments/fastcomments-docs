## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| domainToUpdate | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/put_domain_config_response.py)

## उदाहरण

[inline-code-attrs-start title = 'put_domain_config उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.put_domain_config_response import PutDomainConfigResponse
from client.models.update_domain_config_params import UpdateDomainConfigParams
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और इसका डिफ़ॉल्ट https://fastcomments.com है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# क्लाइंट को API सर्वर सुरक्षा नीति के अनुसार
# प्रमाणीकरण और अधिकृति पैरामीटर कॉन्फ़िगर करना आवश्यक है।
# प्रत्येक auth विधि के उदाहरण नीचे दिए गए हैं, अपने auth उपयोग के मामले को
# पूरा करने वाले उदाहरण का उपयोग करें।

# API key प्रमाणीकरण कॉन्फ़िगर करें: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# यदि आवश्यक हो तो API key के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएँ
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain_to_update = 'domain_to_update_example' # str | 
    update_domain_config_params = client.UpdateDomainConfigParams() # UpdateDomainConfigParams | 

    try:
        api_response = api_instance.put_domain_config(tenant_id, domain_to_update, update_domain_config_params)
        print("The response of DefaultApi->put_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->put_domain_config: %s\n" % e)
[inline-code-end]