दस्तावेज़ों को समूहित करके (यदि groupBy दिया गया हो) और कई ऑपरेशन लागू करके एकत्रित करता है।
विभिन्न ऑपरेशन (जैसे sum, countDistinct, avg, आदि) समर्थित हैं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| parentTenantId | string | query | नहीं |  |
| includeStats | boolean | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## उदाहरण

[inline-code-attrs-start title = 'aggregate उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import AggregateOptions
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# होस्ट परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# क्लाइंट को API सर्वर सुरक्षा नीति के अनुसार
# प्रमाणीकरण और अधिकृति पैरामीटर कॉन्फ़िगर करने होंगे।
# प्रत्येक प्रमाणीकरण विधि के उदाहरण नीचे दिए गए हैं, अपने
# प्रमाणीकरण उपयोग के मामले के अनुरूप उदाहरण का उपयोग करें।

# API कुंजी प्राधिकरण कॉन्फ़िगर करें: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# यदि आवश्यक हो तो API कुंजी के लिए उपसर्ग (जैसे Bearer) सेटअप करने के लिए नीचे दिया कोड अनकमेंट करें
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API क्लाइंट के एक इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (वैकल्पिक)
    include_stats = True # bool |  (वैकल्पिक)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, AggregateOptions(parent_tenant_id=parent_tenant_id, include_stats=include_stats))
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]