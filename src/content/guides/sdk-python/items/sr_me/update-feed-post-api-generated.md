## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'update_feed_post Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.feed_post import FeedPost
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и овлашћења
# у складу са безбедносном политиком API сервера.
# Примери за сваки метод аутентификације су наведени доле; користите пример који
# одговара вашем случају коришћења.
# Конфигуришите ауторизацију API кључем: api_key
# Ако је потребно, уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Направите инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    feed_post = client.FeedPost() # FeedPost | 

    try:
        api_response = api_instance.update_feed_post(tenant_id, id, feed_post)
        print("The response of DefaultApi->update_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_feed_post: %s\n" % e)
[inline-code-end]