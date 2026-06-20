## Параметры

| Имя | Тип | Location | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| forceRecalculate | boolean | query | Нет |  |

## Ответ

Возвращает: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_aggregate_question_results_response.py)

## Пример

[inline-code-attrs-start title = 'Пример bulk_aggregate_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_aggregate_question_results_request import BulkAggregateQuestionResultsRequest
from client.models.bulk_aggregate_question_results_response import BulkAggregateQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример,
# который соответствует вашему сценарию использования аутентификации.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (e.g. Bearer) для API ключа, если это необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    bulk_aggregate_question_results_request = client.BulkAggregateQuestionResultsRequest() # BulkAggregateQuestionResultsRequest | 
    force_recalculate = True # bool |  (необязательно)

    try:
        api_response = api_instance.bulk_aggregate_question_results(tenant_id, bulk_aggregate_question_results_request, force_recalculate=force_recalculate)
        print("The response of DefaultApi->bulk_aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->bulk_aggregate_question_results: %s\n" % e)
[inline-code-end]