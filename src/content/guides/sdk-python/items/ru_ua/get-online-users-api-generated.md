В настоящий момент онлайн на странице: люди, чьи websocket-сессии в данный момент подписаны на страницу.
Возвращает anonCount + totalCount (подписчики комнаты в целом, включая анонимных зрителей, которых мы не перечисляем).

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Курсор-тайбрейкер: передайте nextAfterUserId из предыдущего ответа. Требуется, когда указан afterName, чтобы при совпадении имён записи не отбрасывались. |

## Ответ

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Задание host необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Входим в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создаём экземпляр API-класса
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор URL страницы (очищается на стороне сервера).
    after_name = 'after_name_example' # str | Курсор: передайте nextAfterName из предыдущего ответа. (необязательно)
    after_user_id = 'after_user_id_example' # str | Курсор-тайбрейкер: передайте nextAfterUserId из предыдущего ответа. Требуется, когда указан afterName, чтобы при совпадении имён записи не отбрасывались. (необязательно)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]