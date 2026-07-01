## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nee |  |
| externalId | string | query | Nee |  |
| eventType | string | query | Nee |  |
| type | string | query | Nee |  |
| domain | string | query | Nee |  |
| attemptCountGT | number | query | Nee |  |

## Respons

Returns: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld get_pending_webhook_event_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetPendingWebhookEventCountOptions
from client.models.get_pending_webhook_event_count_response import GetPendingWebhookEventCountResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie‑ en autorisatie‑parameters configureren
# in overeenstemming met het beveiligingsbeleid van de API‑server.
# Voorbeelden voor elke authenticatiemethode worden hieronder gegeven; gebruik het voorbeeld dat
# aansluit bij jouw authenticatie‑use‑case.

# Configureer API‑sleutel‑autorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Start een context met een instantie van de API‑client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API‑klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (optioneel)
    external_id = 'external_id_example' # str |  (optioneel)
    event_type = 'event_type_example' # str |  (optioneel)
    type = 'type_example' # str |  (optioneel)
    domain = 'domain_example' # str |  (optioneel)
    attempt_count_gt = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, GetPendingWebhookEventCountOptions(comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt))
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]