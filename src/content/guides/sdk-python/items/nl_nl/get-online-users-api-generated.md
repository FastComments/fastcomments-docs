Momenteel online kijkers van een pagina: personen wiens websocket-sessie momenteel op de pagina geabonneerd is.
Geeft anonCount + totalCount terug (kamerbrede abonnees, inclusief anonieme kijkers die we niet opsommen).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Identificatie van de pagina-URL (schoongemaakt aan de serverzijde). |
| afterName | string | query | Nee | Cursor: geef nextAfterName door uit het vorige antwoord. |
| afterUserId | string | query | Nee | Cursor tiebreaker: geef nextAfterUserId door uit het vorige antwoord. Verplicht wanneer afterName is ingesteld zodat gelijke namen geen items laten vallen. |

## Response

Geeft terug: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld van get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en staat standaard op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identificatie van de pagina-URL (schoongemaakt aan de serverzijde).
    after_name = 'after_name_example' # str | Cursor: geef nextAfterName door uit het vorige antwoord. (optioneel)
    after_user_id = 'after_user_id_example' # str | Cursor tiebreaker: geef nextAfterUserId door uit het vorige antwoord. Verplicht wanneer afterName is ingesteld zodat gelijke namen geen items laten vallen. (optioneel)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]