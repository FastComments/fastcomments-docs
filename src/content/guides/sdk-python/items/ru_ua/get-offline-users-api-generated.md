Предыдущие комментаторы на странице, которые в данный момент НЕ в сети. Отсортировано по displayName.
Используйте это после исчерпания /users/online для отображения раздела "Участники".
Курсорная пагинация по commenterName: сервер проходит частичный индекс {tenantId, urlId, commenterName}
от afterName вперёд с помощью $gt, без затрат $skip.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Разрешение ничьих в курсоре: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда задан afterName, чтобы совпадения по имени не приводили к пропуску записей. |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор URL страницы (очищается на стороне сервера).
    after_name = 'after_name_example' # str | Курсор: передайте nextAfterName из предыдущего ответа. (необязательно)
    after_user_id = 'after_user_id_example' # str | Разрешение ничьих в курсоре: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда задан afterName, чтобы совпадения по имени не приводили к пропуску записей. (необязательно)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]