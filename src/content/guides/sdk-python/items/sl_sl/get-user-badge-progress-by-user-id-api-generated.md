## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | path | Da |  |

## Odgovor

Vrača: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_response.py)

## Primer

[inline-code-attrs-start title = 'get_user_badge_progress_by_user_id Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_response import APIGetUserBadgeProgressResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je izbirno in privzeto je https://fastcomments.com
# Glejte configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre preverjanja pristnosti in avtorizacije
# v skladu s politiko varnosti API strežnika.
# Spodaj so podani primeri za vsak način avtentikacije, uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Konfigurirajte avtorizacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnje, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 

    try:
        api_response = api_instance.get_user_badge_progress_by_user_id(tenant_id, user_id)
        print("The response of DefaultApi->get_user_badge_progress_by_user_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_by_user_id: %s\n" % e)
[inline-code-end]

---