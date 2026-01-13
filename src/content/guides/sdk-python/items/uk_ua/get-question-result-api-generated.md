## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_result200_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_question_result'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_result200_response import GetQuestionResult200Response
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки API-сервера.
# Наведені нижче приклади для кожного методу автентифікації, використайте приклад, 
# що відповідає вашому випадку використання автентифікації.

# Налаштуйте авторизацію через API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад Bearer) для API ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_result(tenant_id, id)
        print("The response of DefaultApi->get_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_result: %s\n" % e)
[inline-code-end]