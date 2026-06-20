---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Response

Vraća: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_user_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer create_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_user_body import CreateTenantUserBody
from client.models.create_tenant_user_response import CreateTenantUserResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumijeva se https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Primjeri za svaku metodu autentifikacije su navedeni ispod, upotrijebite primjer koji
# zadovoljava vaš slučaj upotrebe autentifikacije.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod kako biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_user_body = client.CreateTenantUserBody() # CreateTenantUserBody | 

    try:
        api_response = api_instance.create_tenant_user(tenant_id, create_tenant_user_body)
        print("The response of DefaultApi->create_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_user: %s\n" % e)
[inline-code-end]

---