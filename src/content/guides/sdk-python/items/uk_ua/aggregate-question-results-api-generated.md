## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| questionId | string | query | Ні |  |
| questionIds | array | query | Ні |  |
| urlId | string | query | Ні |  |
| timeBucket | string | query | Ні |  |
| startDate | string | query | Ні |  |
| forceRecalculate | boolean | query | Ні |  |

## Відповідь

Повертає: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results200_response.py)

## Приклад

[inline-code-attrs-start title = 'aggregate_question_results Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_question_results200_response import AggregateQuestionResults200Response
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове й за замовчуванням дорівнює https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри аутентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу автентифікації наведені нижче, використайте приклад, який
# відповідає вашому сценарію автентифікації.

# Налаштуйте авторизацію за API ключем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрийте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (необов'язково)
    question_ids = ['question_ids_example'] # List[str] |  (необов'язково)
    url_id = 'url_id_example' # str |  (необов'язково)
    time_bucket = client.AggregateTimeBucket() # AggregateTimeBucket |  (необов'язково)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (необов'язково)
    force_recalculate = True # bool |  (необов'язково)

    try:
        api_response = api_instance.aggregate_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, time_bucket=time_bucket, start_date=start_date, force_recalculate=force_recalculate)
        print("The response of DefaultApi->aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate_question_results: %s\n" % e)
[inline-code-end]