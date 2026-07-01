Past kommentatorer på siden som IKKE er online i øjeblikket. Sorteret efter displayName.  
Brug dette efter at have udtømt /users/online for at gengive en "Members"-sektion.  
Cursor-paginering på commenterName: serveren går den delvise {tenantId, urlId, commenterName}  
indeks fra afterName fremad via $gt, ingen $skip-omkostning.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Side-URL-identifikator (rengjort på serveren). |
| afterName | string | query | Nej | Cursor: send nextAfterName fra det foregående svar. |
| afterUserId | string | query | Nej | Cursor-tiebreaker: send nextAfterUserId fra det foregående svar. Påkrævet når afterName er angivet så navnekoblinger ikke udelader poster. |

## Svar

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_offline_users Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Definering af værten er valgfri og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Indtast en kontekst med en forekomst af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en forekomst af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Side-URL-identifikator (rengjort på serveren).
    after_name = 'after_name_example' # str | Cursor: send nextAfterName fra det foregående svar. (optional)
    after_user_id = 'after_user_id_example' # str | Cursor-tiebreaker: send nextAfterUserId fra det foregående svar. Påkrævet når afterName er angivet så navnekoblinger ikke udelader poster. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]