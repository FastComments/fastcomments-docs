## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| updateComments | boolean | query | Ні |  |

## Відповідь

Повертає: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/put_sso_user_api_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад put_sso_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.put_sso_user_api_response import PutSSOUserAPIResponse
from client.models.update_apisso_user_data import UpdateAPISSOUserData
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове й за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Нижче наведені приклади для кожного методу автентифікації — використайте той,
# який відповідає вашому випадку використання.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо це необхідно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст, використовуючи екземпляр клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_apisso_user_data = client.UpdateAPISSOUserData() # UpdateAPISSOUserData | 
    update_comments = True # bool |  (необов'язково)

    try:
        api_response = api_instance.put_sso_user(tenant_id, id, update_apisso_user_data, update_comments=update_comments)
        print("The response of DefaultApi->put_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->put_sso_user: %s\n" % e)
[inline-code-end]