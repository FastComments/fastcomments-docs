---
## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |

## Відповідь

Повертає: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад create_tenant'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_body import CreateTenantBody
from client.models.create_tenant_response import CreateTenantResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням встановлено на https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт має налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки серверу API.
# Приклади для кожного методу автентифікації наведено нижче, використайте приклад, який
# відповідає вашому сценарію автентифікації.

# Налаштуйте авторизацію за API-ключем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_body = client.CreateTenantBody() # CreateTenantBody | 

    try:
        api_response = api_instance.create_tenant(tenant_id, create_tenant_body)
        print("The response of DefaultApi->create_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant: %s\n" % e)
[inline-code-end]

---