req
tenantId
afterId

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| afterId | string | query | Nie |  |
| limit | integer | query | Nie |  |
| tags | array | query | Nie |  |
| sso | string | query | Nie |  |
| isCrawler | boolean | query | Nie |  |
| includeUserInfo | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_public200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_feed_posts_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts_public200_response import GetFeedPostsPublic200Response
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (opcjonalne)
    limit = 56 # int |  (opcjonalne)
    tags = ['tags_example'] # List[str] |  (opcjonalne)
    sso = 'sso_example' # str |  (opcjonalne)
    is_crawler = True # bool |  (opcjonalne)
    include_user_info = True # bool |  (opcjonalne)

    try:
        api_response = api_instance.get_feed_posts_public(tenant_id, after_id=after_id, limit=limit, tags=tags, sso=sso, is_crawler=is_crawler, include_user_info=include_user_info)
        print("The response of PublicApi->get_feed_posts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_public: %s\n" % e)
[inline-code-end]