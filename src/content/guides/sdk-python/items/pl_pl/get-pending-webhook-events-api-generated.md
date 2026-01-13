## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| commentId | string | query | Nie |  |
| externalId | string | query | Nie |  |
| eventType | string | query | Nie |  |
| type | string | query | Nie |  |
| domain | string | query | Nie |  |
| attemptCountGT | number | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events200_response.py)

## Przykład

[inline-code-attrs-start title = 'get_pending_webhook_events Przykład'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_events200_response import GetPendingWebhookEvents200Response
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz plik configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Przykłady dla każdej metody uwierzytelniania znajdują się poniżej — użyj przykładu, który
# odpowiada Twojemu przypadkowi użycia uwierzytelniania.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (optional)
    external_id = 'external_id_example' # str |  (optional)
    event_type = 'event_type_example' # str |  (optional)
    type = 'type_example' # str |  (optional)
    domain = 'domain_example' # str |  (optional)
    attempt_count_gt = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip)
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]

---