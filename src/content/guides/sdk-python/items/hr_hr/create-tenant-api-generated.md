---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | upit | Da |  |

## Odgovor

Vraća: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_response.py)

## Primjer

[inline-code-attrs-start title = 'create_tenant Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_body import CreateTenantBody
from client.models.create_tenant_response import CreateTenantResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu s politikom sigurnosti API servera.
# Primjeri za svaku metodu autentikacije navedeni su dolje, koristite primjer koji
# zadovoljava vaš slučaj upotrebe autentikacije.
# Konfigurirajte autorizaciju pomoću API ključa: api_key
# Otkomentirajte dolje kako biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_body = client.CreateTenantBody() # CreateTenantBody | 

    try:
        api_response = api_instance.create_tenant(tenant_id, create_tenant_body)
        print("The response of DefaultApi->create_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant: %s\n" % e)
[inline-code-end]

---