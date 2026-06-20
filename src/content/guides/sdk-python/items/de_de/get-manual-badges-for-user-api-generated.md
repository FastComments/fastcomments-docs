## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Nein |  |
| commentId | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_manual_badges_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_manual_badges_for_user Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_manual_badges_response import GetUserManualBadgesResponse
from client.rest import ApiException
from pprint import pprint

# Das Festlegen des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Einen Kontext mit einer Instanz des API-Clients öffnen
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.ModerationApi(api_client)
    badges_user_id = 'badges_user_id_example' # str |  (optional)
    comment_id = 'comment_id_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_manual_badges_for_user(badges_user_id=badges_user_id, comment_id=comment_id, sso=sso)
        print("The response of ModerationApi->get_manual_badges_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_manual_badges_for_user: %s\n" % e)
[inline-code-end]