## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| sendEmail | string | query | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_moderator'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове; за замовчуванням використовується https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт має налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу автентифікації наведені нижче; використайте той
# що відповідає вашому випадку використання автентифікації.

# Налаштуйте авторизацію за допомогою API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    send_email = 'send_email_example' # str |  (необов'язково)

    try:
        api_response = api_instance.delete_moderator(tenant_id, id, send_email=send_email)
        print("The response of DefaultApi->delete_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_moderator: %s\n" % e)
[inline-code-end]