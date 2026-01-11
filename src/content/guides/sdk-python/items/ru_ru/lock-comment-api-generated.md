## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| broadcastId | string | query | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`LockComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/lock_comment200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример lock_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.lock_comment200_response import LockComment200Response
from client.rest import ApiException
from pprint import pprint

# Задание host необязательно и по умолчанию используется https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.lock_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->lock_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->lock_comment: %s\n" % e)
[inline-code-end]