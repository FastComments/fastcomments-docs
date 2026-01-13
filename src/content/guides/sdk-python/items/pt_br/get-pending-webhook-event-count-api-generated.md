## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| commentId | string | query | Não |  |
| externalId | string | query | Não |  |
| eventType | string | query | Não |  |
| type | string | query | Não |  |
| domain | string | query | Não |  |
| attemptCountGT | number | query | Não |  |

## Resposta

Retorna: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count200_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_pending_webhook_event_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_event_count200_response import GetPendingWebhookEventCount200Response
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
    comment_id = 'comment_id_example' # str |  (opcional)
    external_id = 'external_id_example' # str |  (opcional)
    event_type = 'event_type_example' # str |  (opcional)
    type = 'type_example' # str |  (opcional)
    domain = 'domain_example' # str |  (opcional)
    attempt_count_gt = 3.4 # float |  (opcional)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt)
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]

---