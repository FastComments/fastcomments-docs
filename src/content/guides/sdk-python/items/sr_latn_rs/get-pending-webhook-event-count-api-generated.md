## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | query | Ne |  |
| externalId | string | query | Ne |  |
| eventType | string | query | Ne |  |
| type | string | query | Ne |  |
| domain | string | query | Ne |  |
| attemptCountGT | number | query | Ne |  |

## Odgovor

Vraća: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_pending_webhook_event_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_event_count_response import GetPendingWebhookEventCountResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa politikom bezbednosti API servera.
# Primeri za svaki metod autentifikacije su dati ispod, koristite primer koji
# zadovoljava vaš slučaj upotrebe autentifikacije.

# Konfigurišite autorizaciju putem API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Napravite instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (neobavezno)
    external_id = 'external_id_example' # str |  (neobavezno)
    event_type = 'event_type_example' # str |  (neobavezno)
    type = 'type_example' # str |  (neobavezno)
    domain = 'domain_example' # str |  (neobavezno)
    attempt_count_gt = 3.4 # float |  (neobavezno)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt)
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]

---