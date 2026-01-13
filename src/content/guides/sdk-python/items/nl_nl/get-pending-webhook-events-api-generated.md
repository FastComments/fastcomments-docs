## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nee |  |
| externalId | string | query | Nee |  |
| eventType | string | query | Nee |  |
| type | string | query | Nee |  |
| domain | string | query | Nee |  |
| attemptCountGT | number | query | Nee |  |
| skip | number | query | Nee |  |

## Antwoord

Retourneert: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_pending_webhook_events Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_events200_response import GetPendingWebhookEvents200Response
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard ingesteld op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode staan hieronder; gebruik het voorbeeld dat
# voldoet aan uw auth-gebruikssituatie.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal hieronder de commentaar weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (optioneel)
    external_id = 'external_id_example' # str |  (optioneel)
    event_type = 'event_type_example' # str |  (optioneel)
    type = 'type_example' # str |  (optioneel)
    domain = 'domain_example' # str |  (optioneel)
    attempt_count_gt = 3.4 # float |  (optioneel)
    skip = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip)
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]