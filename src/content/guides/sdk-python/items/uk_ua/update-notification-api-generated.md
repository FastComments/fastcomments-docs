## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Response

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Example

[inline-code-attrs-start title = 'Приклад update_notification'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_notification_body import UpdateNotificationBody
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки серверу API.
# Нижче наведені приклади для кожного методу автентифікації, використайте той,
# який відповідає вашому випадку використання.

# Налаштуйте авторизацію за допомогою API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрийте контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_notification_body = client.UpdateNotificationBody() # UpdateNotificationBody | 
    user_id = 'user_id_example' # str |  (optional)

    try:
        api_response = api_instance.update_notification(tenant_id, id, update_notification_body, user_id=user_id)
        print("The response of DefaultApi->update_notification:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_notification: %s\n" % e)
[inline-code-end]