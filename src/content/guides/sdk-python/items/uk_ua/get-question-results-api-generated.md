## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| startDate | string | query | No |  |
| questionId | string | query | No |  |
| questionIds | string | query | No |  |
| skip | number | query | No |  |

## Відповідь

Повертає: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results200_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_results200_response import GetQuestionResults200Response
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу автентифікації наведені нижче, використайте той
# що відповідає вашому сценарію автентифікації.

# Налаштуйте авторизацію за API-ключем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрийте контекст із екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (необов'язково)
    user_id = 'user_id_example' # str |  (необов'язково)
    start_date = 'start_date_example' # str |  (необов'язково)
    question_id = 'question_id_example' # str |  (необов'язково)
    question_ids = 'question_ids_example' # str |  (необов'язково)
    skip = 3.4 # float |  (необов'язково)

    try:
        api_response = api_instance.get_question_results(tenant_id, url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip)
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]

---