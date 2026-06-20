Прошлые комментаторы на странице, которые сейчас НЕ в сети. Отсортированы по displayName.
Используйте это после исчерпания /users/online для отображения раздела «Members».
Постраничная пагинация курсором по commenterName: сервер проходит по частичному {tenantId, urlId, commenterName}
индексу от afterName вперёд с использованием $gt, без затрат на $skip.

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на сервере). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Критерий разрешения ничьей для курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда задан afterName, чтобы записи с одинаковыми именами не выпадали. |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Задание хоста необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Вход в контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор URL страницы (очищается на сервере).
    after_name = 'after_name_example' # str | Курсор: передайте nextAfterName из предыдущего ответа. (необязательно)
    after_user_id = 'after_user_id_example' # str | Критерий разрешения ничьей для курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда задан afterName, чтобы записи с одинаковыми именами не выпадали. (необязательно)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]