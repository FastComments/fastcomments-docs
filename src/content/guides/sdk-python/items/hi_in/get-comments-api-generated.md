## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| skip | integer | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |
| contextUserId | string | query | नहीं |  |
| hashTag | string | query | नहीं |  |
| parentId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| fromDate | integer | query | नहीं |  |
| toDate | integer | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## उदाहरण

[inline-code-attrs-start title = 'get_comments उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetCommentsOptions
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# क्लाइंट को API सर्वर की सुरक्षा नीति के अनुसार
# प्रमाणीकरण और अधिकृति पैरामीटर कॉन्फ़िगर करना आवश्यक है।
# प्रत्येक ऑथ मेथड के उदाहरण नीचे दिए गए हैं, अपने ऑथ यूज़ केस के
# अनुरूप उदाहरण का उपयोग करें।

# API की अधिकृति कॉन्फ़िगर करें: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएँ
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (वैकल्पिक)
    limit = 56 # int |  (वैकल्पिक)
    skip = 56 # int |  (वैकल्पिक)
    as_tree = True # bool |  (वैकल्पिक)
    skip_children = 56 # int |  (वैकल्पिक)
    limit_children = 56 # int |  (वैकल्पिक)
    max_tree_depth = 56 # int |  (वैकल्पिक)
    url_id = 'url_id_example' # str |  (वैकल्पिक)
    user_id = 'user_id_example' # str |  (वैकल्पिक)
    anon_user_id = 'anon_user_id_example' # str |  (वैकल्पिक)
    context_user_id = 'context_user_id_example' # str |  (वैकल्पिक)
    hash_tag = 'hash_tag_example' # str |  (वैकल्पिक)
    parent_id = 'parent_id_example' # str |  (वैकल्पिक)
    direction = client.SortDirections() # SortDirections |  (वैकल्पिक)
    from_date = 56 # int |  (वैकल्पिक)
    to_date = 56 # int |  (वैकल्पिक)

    try:
        api_response = api_instance.get_comments(tenant_id, GetCommentsOptions(page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date))
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]