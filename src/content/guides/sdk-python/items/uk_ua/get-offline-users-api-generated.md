Коментатори, які раніше залишали коментарі на сторінці, але наразі НЕ в мережі. Відсортовані за displayName.
Використовуйте це після вичерпання /users/online для відображення розділу «Учасники».
Курсорна пагінація за commenterName: сервер проходить частковим індексом {tenantId, urlId, commenterName}
від afterName вперед за допомогою $gt, без витрат на $skip.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Ідентифікатор URL сторінки (очищається на сервері). |
| afterName | string | query | Ні | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | Ні | Додатковий критерій курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язковий, коли вказано afterName, щоб записи з однаковими іменами не втрачалися. |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Ідентифікатор URL сторінки (очищається на сервері).
    after_name = 'after_name_example' # str | Курсор: передайте nextAfterName з попередньої відповіді. (optional)
    after_user_id = 'after_user_id_example' # str | Додатковий критерій курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язковий, коли вказано afterName, щоб записи з однаковими іменами не втрачалися. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]