req
tenantId
afterId

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| afterId | string | query | Ne |  |
| limit | integer | query | Ne |  |
| tags | array | query | Ne |  |

## Odgovor

Vraća: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_response.py)

## Primjer

[inline-code-attrs-start title = 'get_feed_posts Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts_response import GetFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
# The client must configure the authentication and authorization parameters
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaku metodu autentikacije su navedeni u nastavku, upotrijebite primjer koji
# satisfies your auth use case.
# Configure API key authorization: api_key
# Otkomentirajte dolje za postavljanje prefiksa (npr. Bearer) za API ključ, ako je potrebno
# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (neobavezno)
    limit = 56 # int |  (neobavezno)
    tags = ['tags_example'] # List[str] |  (neobavezno)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, after_id=after_id, limit=limit, tags=tags)
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]