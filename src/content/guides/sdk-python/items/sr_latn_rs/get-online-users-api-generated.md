Currently‑online gledatelji stranice: ljudi čija je websocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnici na čitavu sobu, uključujući anonimne gledatelje koje ne nabrajamo).

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes | |
| urlId | string | query | Yes | Identifikator URL‑a stranice (čišćen na serveru). |
| afterName | string | query | No | Kursor: prosledi nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor razdvajač: prosledi nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se izbeglo propuštanje unosa pri istim imenima. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'get_online_users Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje host‑a je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifikator URL‑a stranice (čišćen na serveru).
    after_name = 'after_name_example' # str | Kursor: prosledi nextAfterName iz prethodnog odgovora. (opciono)
    after_user_id = 'after_user_id_example' # str | Kursor razdvajač: prosledi nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se izbeglo propuštanje unosa pri istim imenima. (opciono)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]