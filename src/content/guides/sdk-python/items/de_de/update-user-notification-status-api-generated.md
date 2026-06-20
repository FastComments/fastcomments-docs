## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| notificationId | string | path | Ja |  |
| newStatus | string | path | Ja |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_status_response.py)

## Beispiel

[inline-code-attrs-start title = 'update_user_notification_status Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_status_response import UpdateUserNotificationStatusResponse
from client.rest import ApiException
from pprint import pprint

# Die Definition des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Einen Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Eine Instanz der API-Klasse erstellen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    notification_id = 'notification_id_example' # str | 
    new_status = 'new_status_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.update_user_notification_status(tenant_id, notification_id, new_status, sso=sso)
        print("The response of PublicApi->update_user_notification_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_status: %s\n" % e)
[inline-code-end]