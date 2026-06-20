## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| userId | string | query | Nein |  |
| direction | string | query | Nein |  |
| repliesToUserId | string | query | Nein |  |
| page | number | query | Nein |  |
| includei10n | boolean | query | Nein |  |
| locale | string | query | Nein |  |
| isCrawler | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
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
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (optional)
    direction = client.SortDirections() # SortDirections |  (optional)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (optional)
    page = 3.4 # float |  (optional)
    includei10n = True # bool |  (optional)
    locale = 'locale_example' # str |  (optional)
    is_crawler = True # bool |  (optional)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]