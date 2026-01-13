## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 아니요 |  |
| userId | string | query | 아니요 |  |
| startDate | string | query | 아니요 |  |
| questionId | string | query | 아니요 |  |
| questionIds | string | query | 아니요 |  |
| skip | number | query | 아니요 |  |

## 응답

반환: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results200_response.py)

## 예제

[inline-code-attrs-start title = 'get_question_results 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_results200_response import GetQuestionResults200Response
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (선택사항)
    user_id = 'user_id_example' # str |  (선택사항)
    start_date = 'start_date_example' # str |  (선택사항)
    question_id = 'question_id_example' # str |  (선택사항)
    question_ids = 'question_ids_example' # str |  (선택사항)
    skip = 3.4 # float |  (선택사항)

    try:
        api_response = api_instance.get_question_results(tenant_id, url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip)
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]

---