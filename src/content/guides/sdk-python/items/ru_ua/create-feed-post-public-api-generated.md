## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | путь | Да |  |
| broadcastId | string | строка запроса | Нет |  |
| sso | string | строка запроса | Нет |  |

## Ответ

Возвращает: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_post_response.py)

## Пример

[inline-code-attrs-start title = 'Пример create_feed_post_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_post_response import CreateFeedPostResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно, по умолчанию используется https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Откройте контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.create_feed_post_public(tenant_id, create_feed_post_params, broadcast_id=broadcast_id, sso=sso)
        print("The response of PublicApi->create_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_feed_post_public: %s\n" % e)
[inline-code-end]

---