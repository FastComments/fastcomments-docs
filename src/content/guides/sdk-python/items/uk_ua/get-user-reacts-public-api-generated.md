---
## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| postIds | array | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/user_reacts_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_user_reacts_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.user_reacts_response import UserReactsResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням дорівнює https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_ids = ['post_ids_example'] # List[str] |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.get_user_reacts_public(tenant_id, post_ids=post_ids, sso=sso)
        print("The response of PublicApi->get_user_reacts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_reacts_public: %s\n" % e)
[inline-code-end]

---