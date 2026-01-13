## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_package200_response.py)

## Primer

[inline-code-attrs-start title = 'create_tenant_package Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_package200_response import CreateTenantPackage200Response
from client.models.create_tenant_package_body import CreateTenantPackageBody
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih nastavitev.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre za preverjanje pristnosti in avtorizacijo
# v skladu s politiko varnosti strežnika API.
# Spodaj so na voljo primeri za vsako metodo avtentikacije, uporabite primer,
# ki ustreza vašemu primeru uporabe avtentikacije.

# Nastavite avtentikacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_package_body = client.CreateTenantPackageBody() # CreateTenantPackageBody | 

    try:
        api_response = api_instance.create_tenant_package(tenant_id, create_tenant_package_body)
        print("The response of DefaultApi->create_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_package: %s\n" % e)
[inline-code-end]

---