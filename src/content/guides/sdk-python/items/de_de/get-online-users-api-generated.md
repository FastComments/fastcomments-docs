Derzeit online betrachtende Besucher einer Seite: Personen, deren Websocket‑Sitzung gerade die Seite abonniert hat.  
Gibt anonCount + totalCount zurück (raumweite Abonnenten, einschließlich anonymer Betrachter, die wir nicht aufzählen).

## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|---------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Seiten‑URL‑Kennung (serverseitig bereinigt). |
| afterName | string | query | Nein | Cursor: übergebe nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | Nein | Cursor‑Tie‑Breaker: übergebe nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Namens‑Gleichstände keine Einträge verlieren. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'Beispiel get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definieren des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Betritt einen Kontext mit einer Instanz des API‑Clients
with client.ApiClient(configuration) as api_client:
    # Erstelle eine Instanz der API‑Klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Seiten‑URL‑Kennung (serverseitig bereinigt).
    after_name = 'after_name_example' # str | Cursor: übergebe nextAfterName aus der vorherigen Antwort. (optional)
    after_user_id = 'after_user_id_example' # str | Cursor‑Tie‑Breaker: übergebe nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Namens‑Gleichstände keine Einträge verlieren. (optional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("Die Antwort von PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Ausnahme beim Aufruf von PublicApi->get_online_users: %s\n" % e)
[inline-code-end]