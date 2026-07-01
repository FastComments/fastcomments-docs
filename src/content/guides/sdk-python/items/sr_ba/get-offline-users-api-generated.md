Past komentatori na stranici koji NISU trenutno online. Sortirani po displayName.  
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije „Members“.  
Kursor paginacija po commenterName: server prolazi kroz parcijalni {tenantId, urlId, commenterName} indeks od afterName naprijed putem $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL‑adrese stranice (čišćen na serveru). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor tiebreaker: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je postavljen afterName kako se ne bi izostavile stavke pri istim imenima. |

## Odgovor

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumijevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str |
    url_id = 'url_id_example' # str | Identifikator URL‑adrese stranice (čišćen na serveru).
    after_name = 'after_name_example' # str | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (opcionalno)
    after_user_id = 'after_user_id_example' # str | Kursor tiebreaker: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je postavljen afterName kako se ne bi izostavile stavke pri istim imenima. (opcionalno)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]