## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| skip | number | query | Ne |  |

## Odgovor

Vrne: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_users200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_tenant_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_users200_response import GetTenantUsers200Response
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je izbirna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre avtentikacije in avtorizacije
# v skladu s politiko varnosti API strežnika.
# Primeri za vsako metodo avtentikacije so navedeni spodaj, uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnjo vrstico, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenant_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_users: %s\n" % e)
[inline-code-end]