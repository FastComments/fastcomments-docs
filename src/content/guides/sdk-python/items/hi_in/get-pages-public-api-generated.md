किसी टेनेंट के लिए पेजों की सूची लौटाता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी रूम सूची भरने के लिए उपयोग किया जाता है।
प्रत्येक पेज के लिए resolved custom config पर `enableFChat` true होना आवश्यक है।
SSO की आवश्यकता वाले पेजों को अनुरोध करने वाले उपयोगकर्ता के group access के आधार पर फ़िल्टर किया जाता है।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| cursor | string | query | नहीं | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा हुआ। |
| limit | integer | query | नहीं | 1..200, डिफ़ॉल्ट 50 |
| q | string | query | नहीं | वैकल्पिक, केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर। |
| sortBy | string | query | नहीं | सॉर्ट क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सर्वाधिक कमेंट पहले), या `title` (वर्णानुक्रम)। |
| hasComments | boolean | query | नहीं | यदि true है, तो केवल कम से कम एक कमेंट वाले पेज लौटाएँ। |

## प्रतिक्रिया

लौटाता है: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## उदाहरण

[inline-code-attrs-start title = 'get_pages_public उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# होस्ट परिभाषित करना वैकल्पिक है और यह https://fastcomments.com पर डिफ़ॉल्ट होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएँ
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा हुआ। (optional)
    limit = 56 # int | 1..200, डिफ़ॉल्ट 50 (optional)
    q = 'q_example' # str | वैकल्पिक, केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर। (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | सॉर्ट क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सर्वाधिक कमेंट पहले), या `title` (वर्णानुक्रम)। (optional)
    has_comments = True # bool | यदि true है, तो केवल कम से कम एक कमेंट वाले पेज लौटाएँ। (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]