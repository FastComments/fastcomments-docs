## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |

## Odgovor

Vrne: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count_response.py)

## Primer

[inline-code-attrs-start title = 'get_pending_webhook_event_count Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetPendingWebhookEventCountOptions
from client.models.get_pending_webhook_event_count_response import GetPendingWebhookEventCountResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre avtentikacije in avtorizacije
# v skladu z varnostno politiko strežnika API.
# Primeri za vsako metodo avtentikacije so podani spodaj, uporabite primer, ki
# zadovoljuje vaš primer uporabe avtentikacije.

# Nastavi avtorizacijo s ključem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za ključ API, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst s primerekom API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvari primerek API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (neobvezno)
    external_id = 'external_id_example' # str |  (neobvezno)
    event_type = 'event_type_example' # str |  (neobvezno)
    type = 'type_example' # str |  (neobvezno)
    domain = 'domain_example' # str |  (neobvezno)
    attempt_count_gt = 3.4 # float |  (neobvezno)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, GetPendingWebhookEventCountOptions(comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt))
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]