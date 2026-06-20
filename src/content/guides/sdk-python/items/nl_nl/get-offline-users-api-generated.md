---
Eerdere commentatoren op de pagina die momenteel NIET online zijn. Gesorteerd op displayName.
Gebruik dit nadat /users/online is uitgeput om een "Members"-sectie weer te geven.
Cursor-paginering op commenterName: de server doorloopt de partiële index {tenantId, urlId, commenterName}
vanaf afterName vooruit via $gt, zonder $skip-kosten.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Pagina-URL-identificator (schoongemaakt aan de serverzijde). |
| afterName | string | query | No | Cursor: geef nextAfterName mee uit het vorige antwoord. |
| afterUserId | string | query | No | Cursor-tiebreaker: geef nextAfterUserId mee uit het vorige antwoord. Vereist wanneer afterName is ingesteld zodat naamgelijkheden geen items weg laten vallen. |

## Response

Geeft terug: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Example

[inline-code-attrs-start title = 'get_offline_users Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Pagina-URL-identificator (schoongemaakt aan de serverzijde).
    after_name = 'after_name_example' # str | Cursor: geef nextAfterName mee uit het vorige antwoord. (optioneel)
    after_user_id = 'after_user_id_example' # str | Cursor-tiebreaker: geef nextAfterUserId mee uit het vorige antwoord. Vereist wanneer afterName is ingesteld zodat naamgelijkheden geen items weg laten vallen. (optioneel)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]

---