## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| broadcastId | string | query | Ja |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`LockComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/lock_comment200_response.py)

## Beispiel

[inline-code-attrs-start title = 'lock_comment Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.lock_comment200_response import LockComment200Response
from client.rest import ApiException
from pprint import pprint

# Die Festlegung des Hosts ist optional und standardmäßig auf https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.lock_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->lock_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->lock_comment: %s\n" % e)
[inline-code-end]