---
Agregira dokumente grupišući ih (ako je groupBy naveden) i primjenjujući više operacija.
Podržane su različite operacije (npr. sum, countDistinct, avg itd.).

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| parentTenantId | string | query | No |  |
| includeStats | boolean | query | No |  |

## Odgovor

Vraća: [`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## Primjer

[inline-code-attrs-start title = 'aggregate Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumijeva se https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Ispod su dati primjeri za svaku metodu autentikacije, koristite primjer koji
# odgovara vašem slučaju upotrebe autentikacije.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite dolje da biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (neobavezno)
    include_stats = True # bool |  (neobavezno)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]

---