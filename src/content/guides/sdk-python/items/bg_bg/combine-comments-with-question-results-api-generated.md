## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |
| minValue | number | query | No |  |
| maxValue | number | query | No |  |
| limit | number | query | No |  |

## Отговор

Връща: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за combine_comments_with_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хост е незадължително и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# Примерите за всеки метод за удостоверяване са показани по-долу, използвайте примера който
# отговаря на вашия случай на използване.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментирайте по-долу, за да настроите префикс (напр. Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (незадължително)
    question_ids = ['question_ids_example'] # List[str] |  (незадължително)
    url_id = 'url_id_example' # str |  (незадължително)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (незадължително)
    force_recalculate = True # bool |  (незадължително)
    min_value = 3.4 # float |  (незадължително)
    max_value = 3.4 # float |  (незадължително)
    limit = 3.4 # float |  (незадължително)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]