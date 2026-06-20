## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_page_search_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_search_pages Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_page_search_response import ModerationPageSearchResponse
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
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_search_pages(value=value, sso=sso)
        print("The response of ModerationApi->get_search_pages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_pages: %s\n" % e)
[inline-code-end]