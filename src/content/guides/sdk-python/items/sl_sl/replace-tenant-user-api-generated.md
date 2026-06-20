## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| updateComments | string | query | Ne |  |

## Odgovor

Vrača: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Primer

[inline-code-attrs-start title = 'Primer replace_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.replace_tenant_user_body import ReplaceTenantUserBody
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Glejte configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre preverjanja pristnosti in pooblastil
# v skladu s politiko varnosti strežnika API.
# Spodaj so podani primeri za vsako metodo overjanja; uporabite primer,
# ki ustreza vašemu primeru uporabe.

# Konfigurirajte avtorizacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    replace_tenant_user_body = client.ReplaceTenantUserBody() # ReplaceTenantUserBody | 
    update_comments = 'update_comments_example' # str |  (optional)

    try:
        api_response = api_instance.replace_tenant_user(tenant_id, id, replace_tenant_user_body, update_comments=update_comments)
        print("The response of DefaultApi->replace_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->replace_tenant_user: %s\n" % e)
[inline-code-end]