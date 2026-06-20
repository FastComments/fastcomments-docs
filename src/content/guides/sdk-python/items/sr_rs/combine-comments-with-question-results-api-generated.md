---
## Параметри

| Name | Type | Location | Required | Description |
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

## Одговор

Враћа: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## Пример

[inline-code-attrs-start title = 'Пример combine_comments_with_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Постављање host-а је опционо и подразумева се https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# Клијент мора да конфигурише параметре аутентификације и ауторизације
# in accordance with the API server security policy.
# у складу са безбедносном политиком API сервера.
# Examples for each auth method are provided below, use the example that
# Примери за сваки метод аутентификације су дати испод, користите пример који
# satisfies your auth use case.
# одговара вашем случају коришћења аутентификације.

# Configure API key authorization: api_key
# Конфигуришите ауторизацију помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Откоментујте испод ако је потребно подесити префикс (нпр. Bearer) за API кључ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (опционо)
    question_ids = ['question_ids_example'] # List[str] |  (опционо)
    url_id = 'url_id_example' # str |  (опционо)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (опционо)
    force_recalculate = True # bool |  (опционо)
    min_value = 3.4 # float |  (опционо)
    max_value = 3.4 # float |  (опционо)
    limit = 3.4 # float |  (опционо)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]

---