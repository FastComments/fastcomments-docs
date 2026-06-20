## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Не |  |
| userId | string | query | Не |  |
| startDate | string | query | Не |  |
| questionId | string | query | Не |  |
| questionIds | string | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_results_response import GetQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# Примерите за всеки метод на автентикация са дадени по-долу, използвайте примера, който
# отговаря на вашия случай на използване за автентикация.

# Конфигурирайте удостоверяване чрез API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментирайте реда по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Отворете контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (по избор)
    user_id = 'user_id_example' # str |  (по избор)
    start_date = 'start_date_example' # str |  (по избор)
    question_id = 'question_id_example' # str |  (по избор)
    question_ids = 'question_ids_example' # str |  (по избор)
    skip = 3.4 # float |  (по избор)

    try:
        api_response = api_instance.get_question_results(tenant_id, url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip)
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]