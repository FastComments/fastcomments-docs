## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| questionId | string | query | नहीं |  |
| questionIds | array | query | नहीं |  |
| urlId | string | query | नहीं |  |
| startDate | string | query | नहीं |  |
| forceRecalculate | boolean | query | नहीं |  |
| minValue | number | query | नहीं |  |
| maxValue | number | query | नहीं |  |
| limit | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## उदाहरण

[inline-code-attrs-start title = 'combine_comments_with_question_results उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import CombineCommentsWithQuestionResultsOptions
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और इसका डिफ़ॉल्ट https://fastcomments.com है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# क्लाइंट को API सर्वर की सुरक्षा नीति के अनुसार
# प्रमाणीकरण और ऑथराइज़ेशन पैरामीटर कॉन्फ़िगर करना आवश्यक है।
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
    question_id = 'question_id_example' # str |  (वैकल्पिक)
    question_ids = ['question_ids_example'] # List[str] |  (वैकल्पिक)
    url_id = 'url_id_example' # str |  (वैकल्पिक)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (वैकल्पिक)
    force_recalculate = True # bool |  (वैकल्पिक)
    min_value = 3.4 # float |  (वैकल्पिक)
    max_value = 3.4 # float |  (वैकल्पिक)
    limit = 3.4 # float |  (वैकल्पिक)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, CombineCommentsWithQuestionResultsOptions(question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit))
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]