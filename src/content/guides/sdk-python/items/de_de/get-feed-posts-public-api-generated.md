req
tenantId
afterId

## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |
| sso | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeUserInfo | boolean | query | No |  |

## Antwort

Gibt zurück: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_feed_posts_response.py)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_feed_posts_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_feed_posts_response import PublicFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (optional)
    limit = 56 # int |  (optional)
    tags = ['tags_example'] # List[str] |  (optional)
    sso = 'sso_example' # str |  (optional)
    is_crawler = True # bool |  (optional)
    include_user_info = True # bool |  (optional)

    try:
        api_response = api_instance.get_feed_posts_public(tenant_id, after_id=after_id, limit=limit, tags=tags, sso=sso, is_crawler=is_crawler, include_user_info=include_user_info)
        print("The response of PublicApi->get_feed_posts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_public: %s\n" % e)
[inline-code-end]