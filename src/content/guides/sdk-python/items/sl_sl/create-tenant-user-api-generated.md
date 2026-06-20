## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_user_response.py)

## Primer

[inline-code-attrs-start title = 'Primer create_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_user_body import CreateTenantUserBody
from client.models.create_tenant_user_response import CreateTenantUserResponse
from client.rest import ApiException
from pprint import pprint

# Nastavitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre avtentikacije in avtorizacije
# v skladu z varnostno politiko API strežnika.
# Spodaj so primeri za vsako metodo avtentikacije; uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Konfigurirajte avtorizacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnjo vrstico, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
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