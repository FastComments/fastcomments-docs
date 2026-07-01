Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina-URL-identificatie (server-side opgeschoond). |
| afterName | string | query | Nee | Cursor: geef nextAfterName door van de vorige response. |
| afterUserId | string | query | Nee | Cursor tiebreaker: geef nextAfterUserId door van de vorige response. Vereist wanneer afterName is ingesteld zodat naam‑ties geen vermelding weglaten. |

## Respons

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_online_users Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API‑client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API‑klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str |
    url_id = 'url_id_example' # str | Pagina-URL-identificatie (server-side opgeschoond).
    after_name = 'after_name_example' # str | Cursor: geef nextAfterName door van de vorige response. (optioneel)
    after_user_id = 'after_user_id_example' # str | Cursor tiebreaker: geef nextAfterUserId door van de vorige response. Vereist wanneer afterName is ingesteld zodat naam‑ties geen vermelding weglaten. (optioneel)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]