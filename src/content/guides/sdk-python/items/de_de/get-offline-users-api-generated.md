Frühere Kommentierende auf der Seite, die derzeit NICHT online sind. Sortiert nach displayName.
Verwenden Sie dies, nachdem Sie /users/online erschöpft haben, um einen "Members"-Bereich anzuzeigen.
Cursor-Paginierung auf commenterName: der Server durchläuft den partiellen {tenantId, urlId, commenterName}-Index ab afterName vorwärts mittels $gt, keine $skip-Kosten.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Seiten-URL-Kennung (serverseitig bereinigt). |
| afterName | string | query | No | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | No | Cursor-Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge verloren gehen. |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_offline_users Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Das Festlegen des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Seiten-URL-Kennung (serverseitig bereinigt).
    after_name = 'after_name_example' # str | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. (optional)
    after_user_id = 'after_user_id_example' # str | Cursor-Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge verloren gehen. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]