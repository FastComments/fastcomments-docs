req
tenantId
afterId

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| afterId | string | query | Не |  |
| limit | integer | query | Не |  |
| tags | array | query | Не |  |

## Одговор

Враћа: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts200_response.py)

## Пример

[inline-code-attrs-start title = 'get_feed_posts Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts200_response import GetFeedPosts200Response
from client.rest import ApiException
from pprint import pprint

# Постављање хоста је опционо и подразумева се https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
# Клијент мора да конфигурише параметре аутентификације и овлашћења
# у складу са безбедносном политиком API сервера.
# Испод су примери за сваки метод аутентификације, користите пример који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите овлашћење API кључа: api_key
# Откоменаришите испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Направите инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (необавезно)
    limit = 56 # int |  (необавезно)
    tags = ['tags_example'] # List[str] |  (необавезно)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, after_id=after_id, limit=limit, tags=tags)
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]