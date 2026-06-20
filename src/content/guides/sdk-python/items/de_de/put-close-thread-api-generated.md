## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| urlId | string | query | Yes |  |
| sso | string | query | No |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für put_close_thread'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.ModerationApi(api_client)
    url_id = 'url_id_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.put_close_thread(url_id, sso=sso)
        print("The response of ModerationApi->put_close_thread:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_close_thread: %s\n" % e)
[inline-code-end]