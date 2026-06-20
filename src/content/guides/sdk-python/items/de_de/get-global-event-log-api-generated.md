req
tenantId
urlId
userIdWS

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| userIdWS | string | query | Ja |  |
| startTime | integer | query | Ja |  |
| endTime | integer | query | Nein |  |

## Antwort

Gibt zurück: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_event_log_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_global_event_log Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_event_log_response import GetEventLogResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffne einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstelle eine Instanz der API-Klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id_ws = 'user_id_ws_example' # str | 
    start_time = 56 # int | 
    end_time = 56 # int |  (optional)

    try:
        api_response = api_instance.get_global_event_log(tenant_id, url_id, user_id_ws, start_time, end_time=end_time)
        print("The response of PublicApi->get_global_event_log:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_global_event_log: %s\n" % e)
[inline-code-end]