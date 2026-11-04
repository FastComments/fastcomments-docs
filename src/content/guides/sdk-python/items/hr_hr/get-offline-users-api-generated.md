Prethodni komentatori na stranici koji TRENUTNO NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije "Members".
Cursor paginacija po commenterName: server prolazi kroz parcijalni {tenantId, urlId, commenterName}
indeks od afterName naprijed putem $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očišćen na serveru). |
| afterName | string | query | Ne | Cursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Cursor tiebreaker: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se unosi s istim imenom ne bi izgubili. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer za get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
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
    url_id = 'url_id_example' # str | Identifikator URL-a stranice (očišćen na serveru).
    after_name = 'after_name_example' # str | Cursor: proslijedite nextAfterName iz prethodnog odgovora. (optional)
    after_user_id = 'after_user_id_example' # str | Cursor tiebreaker: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se unosi s istim imenom ne bi izgubili. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]