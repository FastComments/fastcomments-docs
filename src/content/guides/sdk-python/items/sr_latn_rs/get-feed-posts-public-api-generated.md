req
tenantId
afterId

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |
| sso | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeUserInfo | boolean | query | No |  |

## Odgovor

Vraća: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_feed_posts_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_feed_posts_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_feed_posts_response import PublicFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (opciono)
    limit = 56 # int |  (opciono)
    tags = ['tags_example'] # List[str] |  (opciono)
    sso = 'sso_example' # str |  (opciono)
    is_crawler = True # bool |  (opciono)
    include_user_info = True # bool |  (opciono)

    try:
        api_response = api_instance.get_feed_posts_public(tenant_id, after_id=after_id, limit=limit, tags=tags, sso=sso, is_crawler=is_crawler, include_user_info=include_user_info)
        print("The response of PublicApi->get_feed_posts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_public: %s\n" % e)
[inline-code-end]

---