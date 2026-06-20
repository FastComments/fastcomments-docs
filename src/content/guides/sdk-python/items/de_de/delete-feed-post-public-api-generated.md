## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postId | string | path | Ja |  |
| broadcastId | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_feed_post_public_response.py)

## Beispiel

[inline-code-attrs-start title = 'delete_feed_post_public Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_feed_post_public_response import DeleteFeedPostPublicResponse
from client.rest import ApiException
from pprint import pprint

# Das Festlegen des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Einen Kontext mit einer Instanz des API-Clients öffnen
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.delete_feed_post_public(tenant_id, post_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of PublicApi->delete_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_feed_post_public: %s\n" % e)
[inline-code-end]