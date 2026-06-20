## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Antwort

Gibt zurück: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_banned_users_from_comment_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_ban_users_from_comment Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_banned_users_from_comment_response import GetBannedUsersFromCommentResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt
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
        api_response = api_instance.get_ban_users_from_comment(comment_id, sso=sso)
        print("The response of ModerationApi->get_ban_users_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_ban_users_from_comment: %s\n" % e)
[inline-code-end]