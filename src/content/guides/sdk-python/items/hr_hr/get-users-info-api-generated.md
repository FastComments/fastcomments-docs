Masovne informacije o korisnicima za najmoprimca (tenant). Za dane userIds, vraća informacije o prikazu iz User / SSOUser.
Koristi ga widget za komentare za obogaćivanje korisnika koji su upravo pojavili putem presence eventa.
Bez konteksta stranice: privatnost se provodi jednoliko (privatni profili su maskirani).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | userIds odvojeni zarezima. |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## Primjer

[inline-code-attrs-start title = 'get_users_info Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | Comma-delimited userIds.

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]