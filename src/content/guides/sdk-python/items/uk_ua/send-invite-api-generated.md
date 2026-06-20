## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| fromName | string | query | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад send_invite'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням встановлено на https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу автентифікації наведені нижче, використайте приклад, який
# задовольняє ваш випадок використання автентифікації.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (e.g. Bearer) для API ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрийте контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    from_name = 'from_name_example' # str | 

    try:
        api_response = api_instance.send_invite(tenant_id, id, from_name)
        print("The response of DefaultApi->send_invite:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->send_invite: %s\n" % e)
[inline-code-end]