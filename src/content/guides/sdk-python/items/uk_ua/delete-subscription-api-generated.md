## Параметри

| Назва | Тип | Location | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| userId | string | query | Ні |  |

## Відповідь

Повертає: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_subscription_api_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_subscription'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_subscription_api_response import DeleteSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням встановлено на https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу автентифікації наведено нижче, використайте той приклад, який
# відповідає вашому випадку використання автентифікації.

# Налаштуйте авторизацію за допомогою API ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрийте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (необов'язково)

    try:
        api_response = api_instance.delete_subscription(tenant_id, id, user_id=user_id)
        print("The response of DefaultApi->delete_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_subscription: %s\n" % e)
[inline-code-end]