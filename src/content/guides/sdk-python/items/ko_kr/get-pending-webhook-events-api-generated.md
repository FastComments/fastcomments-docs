## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | query | 아니오 |  |
| externalId | string | query | 아니오 |  |
| eventType | string | query | 아니오 |  |
| type | string | query | 아니오 |  |
| domain | string | query | 아니오 |  |
| attemptCountGT | number | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events200_response.py)

## 예제

[inline-code-attrs-start title = 'get_pending_webhook_events 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_events200_response import GetPendingWebhookEvents200Response
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
    comment_id = 'comment_id_example' # str |  (선택 사항)
    external_id = 'external_id_example' # str |  (선택 사항)
    event_type = 'event_type_example' # str |  (선택 사항)
    type = 'type_example' # str |  (선택 사항)
    domain = 'domain_example' # str |  (선택 사항)
    attempt_count_gt = 3.4 # float |  (선택 사항)
    skip = 3.4 # float |  (선택 사항)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip)
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]