## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| meta | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants200_response.py)

## Primjer

[inline-code-attrs-start title = 'get_tenants Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenants200_response import GetTenants200Response
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaku metodu autentikacije navedeni su dolje; koristite onaj
# koji odgovara vašem scenariju autentikacije.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uklonite komentar ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (neobavezno)
    skip = 3.4 # float |  (neobavezno)

    try:
        api_response = api_instance.get_tenants(tenant_id, meta=meta, skip=skip)
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]