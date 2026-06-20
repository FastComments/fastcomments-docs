## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | path | Так |  |

## Відповідь

Повертає: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_user_badge_progress_by_user_id'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_response import APIGetUserBadgeProgressResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням дорівнює https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт має налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Нижче наведені приклади для кожного методу аутентифікації, використайте той приклад, який
# відповідає вашому випадку використання автентифікації.

# Налаштуйте авторизацію за допомогою ключа API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для ключа API, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 

    try:
        api_response = api_instance.get_user_badge_progress_by_user_id(tenant_id, user_id)
        print("The response of DefaultApi->get_user_badge_progress_by_user_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_by_user_id: %s\n" % e)
[inline-code-end]