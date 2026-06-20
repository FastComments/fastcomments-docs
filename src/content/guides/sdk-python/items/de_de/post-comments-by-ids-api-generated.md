## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| sso | string | Abfrage | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_child_comments_response.py)

## Beispiel

[inline-code-attrs-start title = 'post_comments_by_ids Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comments_by_ids_params import CommentsByIdsParams
from client.models.moderation_api_child_comments_response import ModerationAPIChildCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Die Festlegung des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.ModerationApi(api_client)
    comments_by_ids_params = client.CommentsByIdsParams() # CommentsByIdsParams | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_comments_by_ids(comments_by_ids_params, sso=sso)
        print("The response of ModerationApi->post_comments_by_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_comments_by_ids: %s\n" % e)
[inline-code-end]