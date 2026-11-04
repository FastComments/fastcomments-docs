Pretekli komentatorji na strani, ki trenutno NISO online. Razvrščeni po displayName.
Uporabite to po izčrpanju /users/online za prikaz razdelka "Members".
Straniščna paginacija po commenterName: strežnik pregleduje delni {tenantId, urlId, commenterName}
indeks od afterName naprej prek $gt, brez stroškov $skip.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-ja strani (očiščen na strežniku). |
| afterName | string | query | Ne | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Kazalni razločevalnik: posredujte nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je nastavljen afterName, da vnosi z istim imenom ne izginejo. |

## Odgovor

Vrne: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
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
    url_id = 'url_id_example' # str | Identifikator URL-ja strani (očiščen na strežniku).
    after_name = 'after_name_example' # str | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. (optional)
    after_user_id = 'after_user_id_example' # str | Kazalni razločevalnik: posredujte nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je nastavljen afterName, da vnosi z istim imenom ne izginejo. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]