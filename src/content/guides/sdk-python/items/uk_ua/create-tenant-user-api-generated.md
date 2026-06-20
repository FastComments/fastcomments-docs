## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |

## Відповідь

Повертає: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_user_response.py)

## Приклад

[inline-code-attrs-start title = 'create_tenant_user Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_user_body import CreateTenantUserBody
from client.models.create_tenant_user_response import CreateTenantUserResponse
from client.rest import ApiException
from pprint import pprint

# Визначення host необов'язкове і за замовчуванням має значення https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт має налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки серверу API.
# Приклади для кожного методу автентифікації наведені нижче — використайте той,
# який відповідає вашому випадку використання.

# Налаштування авторизації за допомогою API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб задати префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрийте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_user_body = client.CreateTenantUserBody() # CreateTenantUserBody | 

    try:
        api_response = api_instance.create_tenant_user(tenant_id, create_tenant_user_body)
        print("The response of DefaultApi->create_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_user: %s\n" % e)
[inline-code-end]