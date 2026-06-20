Масова інформація про користувачів для тенанта. За заданими userIds повертає відображувану інформацію з User / SSOUser.
Використовується віджетом коментарів для доповнення інформації про користувачів, які щойно з’явилися через подію присутності.
Без контексту сторінки: конфіденційність застосовується однаково (приватні профілі маскуються).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | userIds, розділені комами. |

## Відповідь

Повертає: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_users_info'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# Визначення host необов'язкове й за замовчуванням встановлено на https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкриваємо контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створюємо екземпляр API-класу
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | userIds, розділені комами.

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]

---