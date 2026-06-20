## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |
| order | string | query | Ne |  |
| after | number | query | Ne |  |
| before | number | query | Ne |  |

## Odziv

Vrača: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_audit_logs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto je https://fastcomments.com
# Za seznam vseh podprtih konfiguracijskih parametrov glej configuration.py.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre za overjanje in avtorizacijo
# v skladu s varnostno politiko API strežnika.
# Spodaj so primeri za vsak način avtentikacije; uporabi tistega,
# ki ustreza tvojemu primeru uporabe.

# Nastavi avtorizacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentiraj spodnje, če želiš nastaviti predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopi v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvari instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (neobvezno)
    skip = 3.4 # float |  (neobvezno)
    order = client.SORTDIR() # SORTDIR |  (neobvezno)
    after = 3.4 # float |  (neobvezno)
    before = 3.4 # float |  (neobvezno)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]

---