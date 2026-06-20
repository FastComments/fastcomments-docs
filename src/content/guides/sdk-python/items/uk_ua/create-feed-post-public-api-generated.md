## Параметри

| Назва | Type | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | шлях | Так |  |
| broadcastId | string | параметр запиту | Ні |  |
| sso | string | параметр запиту | Ні |  |

## Відповідь

Повертає: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_post_response.py)

## Приклад

[inline-code-attrs-start title = 'create_feed_post_public Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_post_response import CreateFeedPostResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням дорівнює https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрийте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.create_feed_post_public(tenant_id, create_feed_post_params, broadcast_id=broadcast_id, sso=sso)
        print("The response of PublicApi->create_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_feed_post_public: %s\n" % e)
[inline-code-end]