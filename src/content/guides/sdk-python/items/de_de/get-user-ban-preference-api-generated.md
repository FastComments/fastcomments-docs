## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_moderate_get_user_ban_preferences_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_user_ban_preference Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_moderate_get_user_ban_preferences_response import APIModerateGetUserBanPreferencesResponse
from client.rest import ApiException
from pprint import pprint

# Das Festlegen des Hosts ist optional und standardmäßig auf https://fastcomments.com eingestellt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.ModerationApi(api_client)
    sso = 'sso_example' # str | (optional)

    try:
        api_response = api_instance.get_user_ban_preference(sso=sso)
        print("The response of ModerationApi->get_user_ban_preference:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_user_ban_preference: %s\n" % e)
[inline-code-end]