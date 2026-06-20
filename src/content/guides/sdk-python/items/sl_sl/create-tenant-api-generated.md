## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_response.py)

## Primer

[inline-code-attrs-start title = 'create_tenant Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_body import CreateTenantBody
from client.models.create_tenant_response import CreateTenantResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je izbirna in privzeto nastavljena na https://fastcomments.com
# Za seznam vseh podprtih parametrov konfiguracije glejte configuration.py.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre avtentikacije in avtorizacije
# v skladu s politiko varnosti API strežnika.
# Spodaj so navedeni primeri za vsako metodo avtentikacije; uporabite primer,
# ki ustreza vašemu primeru uporabe avtentikacije.

# Konfigurirajte avtorizacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
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