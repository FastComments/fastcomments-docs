Frühere Kommentatoren auf der Seite, die derzeit NICHT online sind. Sortiert nach displayName.
Verwenden Sie dies nach dem Ausschöpfen von /users/online, um einen „Members“-Abschnitt darzustellen.
Cursor-Paginierung über commenterName: Der Server durchläuft den partiellen {tenantId, urlId, commenterName}
Index ab afterName vorwärts über $gt, ohne $skip-Kosten.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Seiten-URL-Identifikator (serverseitig bereinigt). |
| afterName | string | query | Nein | Cursor: nextAfterName aus der vorherigen Antwort übergeben. |
| afterUserId | string | query | Nein | Cursor-Tiebreaker: nextAfterUserId aus der vorherigen Antwort übergeben. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge verlieren. |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_offline_users Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Seiten-URL-Identifikator (serverseitig bereinigt).
    after_name = 'after_name_example' # str | Cursor: nextAfterName aus der vorherigen Antwort übergeben. (optional)
    after_user_id = 'after_user_id_example' # str | Cursor-Tiebreaker: nextAfterUserId aus der vorherigen Antwort übergeben. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge verlieren. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]