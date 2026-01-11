## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | параметр запиту | Так |  |

## Відповідь

Повертає: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_sso_user_api_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад add_sso_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_sso_user_api_response import AddSSOUserAPIResponse
from client.models.create_apisso_user_data import CreateAPISSOUserData
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове й за замовчуванням встановлено на https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Нижче наведено приклади для кожного методу автентифікації, використайте той,
# який відповідає вашому випадку використання.

# Налаштуйте авторизацію за допомогою API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст із екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_apisso_user_data = client.CreateAPISSOUserData() # CreateAPISSOUserData | 

    try:
        api_response = api_instance.add_sso_user(tenant_id, create_apisso_user_data)
        print("The response of DefaultApi->add_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_sso_user: %s\n" % e)
[inline-code-end]

---