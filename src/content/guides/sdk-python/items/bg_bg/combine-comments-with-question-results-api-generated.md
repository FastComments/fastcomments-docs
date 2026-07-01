## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| questionId | string | query | Не |  |
| questionIds | array | query | Не |  |
| urlId | string | query | Не |  |
| startDate | string | query | Не |  |
| forceRecalculate | boolean | query | Не |  |
| minValue | number | query | Не |  |
| maxValue | number | query | Не |  |
| limit | number | query | Не |  |

## Отговор

Връща: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## Пример

[inline-code-attrs-start title = 'combine_comments_with_question_results пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import CombineCommentsWithQuestionResultsOptions
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Определянето на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за автентификация и упълномощаване
# в съответствие с политиката за сигурност на API сървъра.
# Примери за всеки метод за автентификация са предоставени по-долу, използвайте примера, който
# отговаря на вашия случай на използване.

# Конфигурирайте API ключ за упълномощаване: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |
    question_id = 'question_id_example' # str |  (по избор)
    question_ids = ['question_ids_example'] # List[str] |  (по избор)
    url_id = 'url_id_example' # str |  (по избор)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (по избор)
    force_recalculate = True # bool |  (по избор)
    min_value = 3.4 # float |  (по избор)
    max_value = 3.4 # float |  (по избор)
    limit = 3.4 # float |  (по избор)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, CombineCommentsWithQuestionResultsOptions(question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit))
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]