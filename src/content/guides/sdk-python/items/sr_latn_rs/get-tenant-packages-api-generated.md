---
## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_packages_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_tenant_packages'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_packages_response import GetTenantPackagesResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa politikom bezbednosti API servera.
# Primeri za svaku metodu autentifikacije su dati ispod, koristite primer koji
# odgovara vašem slučaju upotrebe autentifikacije.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomenarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenant_packages(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_packages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_packages: %s\n" % e)
[inline-code-end]

---