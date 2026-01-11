req
tenantId
afterId

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| afterId | string | query | Ні |  |
| limit | integer | query | Ні |  |
| tags | array | query | Ні |  |
| sso | string | query | Ні |  |
| isCrawler | boolean | query | Ні |  |
| includeUserInfo | boolean | query | Ні |  |

## Відповідь

Повертає: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_public200_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_feed_posts_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts_public200_response import GetFeedPostsPublic200Response
from client.rest import ApiException
from pprint import pprint

# Визначення host необов'язкове й за замовчуванням — https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрийте контекст із екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (необов'язково)
    limit = 56 # int |  (необов'язково)
    tags = ['tags_example'] # List[str] |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)
    is_crawler = True # bool |  (необов'язково)
    include_user_info = True # bool |  (необов'язково)

    try:
        api_response = api_instance.get_feed_posts_public(tenant_id, after_id=after_id, limit=limit, tags=tags, sso=sso, is_crawler=is_crawler, include_user_info=include_user_info)
        print("The response of PublicApi->get_feed_posts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_public: %s\n" % e)
[inline-code-end]