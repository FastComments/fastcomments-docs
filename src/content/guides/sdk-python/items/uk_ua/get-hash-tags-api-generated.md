## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| page | number | query | Ні |  |

## Відповідь

Повертає: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_hash_tags200_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_hash_tags'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_hash_tags200_response import GetHashTags200Response
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки API-сервера.
# Приклади для кожного методу автентифікації наведені нижче; використайте той приклад, 
# який відповідає вашому випадку використання автентифікації.

# Configure API key authorization: api_key
# Налаштуйте авторизацію за допомогою API-ключа: api_key

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо це необхідно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 3.4 # float |  (необов'язково)

    try:
        api_response = api_instance.get_hash_tags(tenant_id, page=page)
        print("The response of DefaultApi->get_hash_tags:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_hash_tags: %s\n" % e)
[inline-code-end]

---