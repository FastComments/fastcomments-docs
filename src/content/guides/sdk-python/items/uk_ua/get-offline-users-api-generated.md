Колишні коментатори на сторінці, які наразі не в мережі. Відсортовано за displayName.
Використовуйте це після вичерпання /users/online для відображення секції "Members".
Курсорна пагінація по commenterName: сервер проходить частковий індекс {tenantId, urlId, commenterName} починаючи після afterName вперед через $gt, без витрат $skip.

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Page URL identifier (cleaned server-side). |
| afterName | string | query | Ні | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | Ні | Тайбрейкер курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли afterName встановлено, щоб прив'язки за ім'ям не призводили до пропуску записів. |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Визначення host необов'язкове й за замовчуванням має значення https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Створюємо контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створюємо екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Page URL identifier (cleaned server-side).
    after_name = 'after_name_example' # str | Курсор: передайте nextAfterName з попередньої відповіді. (optional)
    after_user_id = 'after_user_id_example' # str | Тайбрейкер курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли afterName встановлено, щоб прив'язки за ім'ям не призводили до пропуску записів. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]

---