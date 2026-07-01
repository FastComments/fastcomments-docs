Past komentari na stranici koji NIJSU trenutno online. Sortirano po displayName.  
Koristite ovo nakon što iscrpite /users/online da biste prikazali sekciju "Members".  
Kursor paginacija po commenterName: server prolazi kroz delimični {tenantId, urlId, commenterName} indeks od afterName napred putem $gt, bez troška $skip.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (čišćen na serveru). |
| afterName | string | query | No | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor razrešavač podataka: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se ne bi izostavili zapisi zbog vezanosti imena. |

## Response

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Example

[inline-code-attrs-start title = 'get_offline_users Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifikator URL-a stranice (čišćen na serveru).
    after_name = 'after_name_example' # str | Kursor: prosledite nextAfterName iz prethodnog odgovora. (opciono)
    after_user_id = 'after_user_id_example' # str | Kursor razrešavač podataka: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se ne bi izostavili zapisi zbog vezanosti imena. (opciono)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]