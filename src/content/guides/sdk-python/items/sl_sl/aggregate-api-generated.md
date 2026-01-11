Agregira dokumente z združevanjem (če je podan groupBy) in izvajanjem več operacij. Podprte so različne operacije (npr. sum, countDistinct, avg, itd.).

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| parentTenantId | string | query | Ne |  |
| includeStats | boolean | query | Ne |  |

## Odgovor

Vrne: [`AggregationResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregation_response.py)

## Primer

[inline-code-attrs-start title = 'Primer aggregate'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregation_request import AggregationRequest
from client.models.aggregation_response import AggregationResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre overjanja in pooblastila
# v skladu z varnostno politiko API strežnika.
# Spodaj so podani primeri za vsako metodo avtentikacije, uporabite primer,
# ki ustreza vašemu primeru uporabe avtentikacije.

# Konfigurirajte avtentikacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnjo vrstico, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (neobvezno)
    include_stats = True # bool |  (neobvezno)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]

---