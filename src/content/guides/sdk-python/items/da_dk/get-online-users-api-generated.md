Aktuelt online brugere af en side: personer hvis websocket-session er tilmeldt siden lige nu.
Returnerer anonCount + totalCount (abonnenter for hele rummet, inklusive anonyme seere, som vi ikke opregner).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Page URL identifier (renset på serversiden). |
| afterName | string | query | Nej | Cursor: send nextAfterName fra det forrige svar. |
| afterUserId | string | query | Nej | Cursor tiebreaker: send nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så elementer med samme navn ikke udelades. |

## Response

Returnerer: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'get_online_users Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri, og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Side-URL-identifikator (renset på serversiden).
    after_name = 'after_name_example' # str | Cursor: send nextAfterName fra det forrige svar. (valgfri)
    after_user_id = 'after_user_id_example' # str | Cursor tiebreaker: send nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så elementer med samme navn ikke udelades. (valgfri)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]