req
tenantId
afterId

## Параметри

| Name | Type | Location | Обавезно | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| afterId | string | query | Не |  |
| limit | integer | query | Не |  |
| tags | array | query | Не |  |
| sso | string | query | Не |  |
| isCrawler | boolean | query | Не |  |
| includeUserInfo | boolean | query | Не |  |

## Одговор

Враћа: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_public200_response.py)

## Пример

[inline-code-attrs-start title = 'get_feed_posts_public Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts_public200_response import GetFeedPostsPublic200Response
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (опционо)
    limit = 56 # int |  (опционо)
    tags = ['tags_example'] # List[str] |  (опционо)
    sso = 'sso_example' # str |  (опционо)
    is_crawler = True # bool |  (опционо)
    include_user_info = True # bool |  (опционо)

    try:
        api_response = api_instance.get_feed_posts_public(tenant_id, after_id=after_id, limit=limit, tags=tags, sso=sso, is_crawler=is_crawler, include_user_info=include_user_info)
        print("The response of PublicApi->get_feed_posts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_public: %s\n" % e)
[inline-code-end]