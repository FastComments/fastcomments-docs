Tidligere kommentatorer på siden, som IKKE er online lige nu. Sorteret efter displayName.
Brug dette efter at have udtømt /users/online for at gengive en "Medlemmer"-sektion.
Cursor-paginering på commenterName: serveren går gennem det delvise {tenantId, urlId, commenterName}-indeks fra afterName frem via $gt, uden $skip-omkostning.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Side URL-identifikator (renset på serversiden). |
| afterName | string | query | No | Cursor: angiv nextAfterName fra det forrige svar. |
| afterUserId | string | query | No | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er sat, så poster med samme navn ikke udelades. |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_offline_users Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# At angive host er valgfrit og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Side URL-identifikator (renset på serversiden).
    after_name = 'after_name_example' # str | Cursor: angiv nextAfterName fra det forrige svar. (valgfri)
    after_user_id = 'after_user_id_example' # str | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er sat, så poster med samme navn ikke udelades. (valgfri)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]