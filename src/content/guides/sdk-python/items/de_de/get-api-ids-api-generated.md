## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Nein |  |
| byIPFromComment | string | query | Nein |  |
| filters | string | query | Nein |  |
| searchFilters | string | query | Nein |  |
| afterId | string | query | Nein |  |
| demo | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comment_ids_response.py)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_api_ids'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comment_ids_response import ModerationAPIGetCommentIdsResponse
from client.rest import ApiException
from pprint import pprint

# Die Definition des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (optional)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (optional)
    filters = 'filters_example' # str |  (optional)
    search_filters = 'search_filters_example' # str |  (optional)
    after_id = 'after_id_example' # str |  (optional)
    demo = True # bool |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_api_ids(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, after_id=after_id, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_ids: %s\n" % e)
[inline-code-end]