## Параметри

| Назва | Тип | Location | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ні |  |

## Відповідь

Повертає: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_hash_tag_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад add_hash_tag'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_hash_tag_body import CreateHashTagBody
from client.models.create_hash_tag_response import CreateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# Клієнт повинен налаштувати параметри автентифікації та авторизації
# in accordance with the API server security policy.
# відповідно до політики безпеки API-сервера.
# Examples for each auth method are provided below, use the example that
# Нижче наведено приклади для кожного методу автентифікації; використовуйте той,
# satisfies your auth use case.
# що відповідає вашому випадку використання автентифікації.

# Configure API key authorization: api_key
# Налаштуйте авторизацію за допомогою API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Відкрийте контекст із екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (необов'язковий)
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (необов'язковий)

    try:
        api_response = api_instance.add_hash_tag(tenant_id=tenant_id, create_hash_tag_body=create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]