## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |

## Відповідь

Повертає: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_pending_webhook_event_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_event_count_response import GetPendingWebhookEventCountResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# у відповідності з політикою безпеки серверу API.
# Нижче наведено приклади для кожного методу автентифікації, використайте той приклад,
# який відповідає вашому сценарію автентифікації.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (необов'язковий)
    external_id = 'external_id_example' # str |  (необов'язковий)
    event_type = 'event_type_example' # str |  (необов'язковий)
    type = 'type_example' # str |  (необов'язковий)
    domain = 'domain_example' # str |  (необов'язковий)
    attempt_count_gt = 3.4 # float |  (необов'язковий)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt)
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]

---