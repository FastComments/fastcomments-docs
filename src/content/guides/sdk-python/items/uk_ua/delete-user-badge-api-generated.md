## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Відповідь

Повертає: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_badge200_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_user_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_badge200_response import UpdateUserBadge200Response
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове; за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт має налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Нижче наведено приклади для кожного методу автентифікації, використайте приклад, який
# відповідає вашому випадку використання автентифікації.

# Налаштуйте авторизацію за допомогою API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Використайте контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_user_badge(tenant_id, id)
        print("The response of DefaultApi->delete_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_user_badge: %s\n" % e)
[inline-code-end]

---