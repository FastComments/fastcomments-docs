Past commenters on the page who are NOT currently online. Gesorteerd op displayName.  
Gebruik dit nadat /users/online is uitgeput om een “Members”‑sectie weer te geven.  
Cursor‑paginering op commenterName: server loopt door de gedeeltelijke {tenantId, urlId, commenterName} index vanaf afterName met $gt, geen $skip‑kosten.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina‑URL‑identificatie (server‑zijde opgeschoond). |
| afterName | string | query | Nee | Cursor: geef nextAfterName door van de vorige response. |
| afterUserId | string | query | Nee | Cursor‑ontknoping: geef nextAfterUserId door van de vorige response. Vereist wanneer afterName is ingesteld zodat naam‑gelijkwaardes geen invoer laten vallen. |

## Respons

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_offline_users Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Pagina-URL-identificatie (server-zijde opgeschoond).
    after_name = 'after_name_example' # str | Cursor: geef nextAfterName door van de vorige response. (optioneel)
    after_user_id = 'after_user_id_example' # str | Cursor-ontknoping: geef nextAfterUserId door van de vorige response. Vereist wanneer afterName is ingesteld zodat naam-gelijkwaardes geen invoer laten vallen. (optioneel)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]