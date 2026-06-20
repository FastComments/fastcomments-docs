## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |
| order | string | query | Ne |  |
| after | number | query | Ne |  |
| before | number | query | Ne |  |

## Odgovor

Vraća: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_audit_logs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i prema zadanim postavkama je https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu s politikom sigurnosti API servera.
# Primjeri za svaki način autentifikacije dani su dolje, upotrijebite primjer koji
# odgovara vašem slučaju upotrebe autentifikacije.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s primjerkom API klijenta
with client.ApiClient(configuration) as api_client:
    # Stvorite primjerak API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)
    order = client.SORTDIR() # SORTDIR |  (optional)
    after = 3.4 # float |  (optional)
    before = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]