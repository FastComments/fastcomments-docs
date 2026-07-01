---
Aktuelle online‑seere af en side: personer, hvis websocket‑session er abonneret på siden lige nu.  
Returnerer anonCount + totalCount (rum‑omfattende abonnenter, inklusiv anonyme seere, som vi ikke tæller op).

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Side‑URL‑identifikator (rengjort på serveren). |
| afterName | string | query | No | Markør: send nextAfterName fra den forrige respons. |
| afterUserId | string | query | No | Markør‑tiebreaker: send nextAfterUserId fra den forrige respons. Påkrævet når afterName er angivet, så navneties ikke tabes. |

## Respons

Returnerer: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_online_users Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definering af værten er valgfri og har standardværdien https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Indtast en kontekst med en forekomst af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en forekomst af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Side‑URL‑identifikator (rengjort på serveren).
    after_name = 'after_name_example' # str | Markør: send nextAfterName fra den forrige respons. (valgfri)
    after_user_id = 'after_user_id_example' # str | Markør‑tiebreaker: send nextAfterUserId fra den forrige respons. Påkrævet når afterName er angivet, så navneties ikke tabes. (valgfri)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]

---