---
## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/post_remove_comment_response.py)

## Beispiel

[inline-code-attrs-start title = 'post_remove_comment Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.post_remove_comment_response import PostRemoveCommentResponse
from client.rest import ApiException
from pprint import pprint

# Die Festlegung des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_remove_comment(comment_id, sso=sso)
        print("The response of ModerationApi->post_remove_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_remove_comment: %s\n" % e)
[inline-code-end]

---