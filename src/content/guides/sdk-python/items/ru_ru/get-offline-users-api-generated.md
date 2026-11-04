Комментаторы страницы, которые сейчас НЕ в сети. Отсортированы по displayName.
Используйте этот метод после исчерпания /users/online для отображения раздела «Участники».
Курсорная пагинация по commenterName: сервер проходит по частичному индексу {tenantId, urlId, commenterName}
от afterName вперёд через $gt, без затрат на $skip.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на сервере). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Дополнительный ключ курсора: передайте nextAfterUserId из предыдущего ответа. Обязателен, когда задан afterName, чтобы записи с одинаковыми именами не терялись. |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Определение хоста необязательно, по умолчанию используется https://fastcomments.com
# Список всех поддерживаемых параметров конфигурации см. в configuration.py.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор URL страницы (очищается на сервере).
    after_name = 'after_name_example' # str | Курсор: передайте nextAfterName из предыдущего ответа. (optional)
    after_user_id = 'after_user_id_example' # str | Дополнительный ключ курсора: передайте nextAfterUserId из предыдущего ответа. Обязателен, когда задан afterName, чтобы записи с одинаковыми именами не терялись. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]