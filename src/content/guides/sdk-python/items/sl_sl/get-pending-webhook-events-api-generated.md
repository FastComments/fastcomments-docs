## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |
| skip | number | query | No |  |

## Odgovor

Vrne: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events_response.py)

## Primer

[inline-code-attrs-start title = 'get_pending_webhook_events Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetPendingWebhookEventsOptions
from client.models.get_pending_webhook_events_response import GetPendingWebhookEventsResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje gostitelja je neobvezno in privzeto nastavljeno na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov nastavitve.
# Odjemalec mora konfigurirati parametre avtentikacije in avtorizacije
# v skladu s politiko varnosti strežnika API.
# Primeri za vsako metodo avtentikacije so navedeni spodaj, uporabite primer,
# ki ustreza vašemu primeru uporabe avtentikacije.

# Konfigurirajte avtorizacijo s ključem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za ključ API, po potrebi
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst s primerek API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite primerek API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (neobvezno)
    external_id = 'external_id_example' # str |  (neobvezno)
    event_type = 'event_type_example' # str |  (neobvezno)
    type = 'type_example' # str |  (neobvezno)
    domain = 'domain_example' # str |  (neobvezno)
    attempt_count_gt = 3.4 # float |  (neobvezno)
    skip = 3.4 # float |  (neobvezno)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, GetPendingWebhookEventsOptions(comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip))
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]