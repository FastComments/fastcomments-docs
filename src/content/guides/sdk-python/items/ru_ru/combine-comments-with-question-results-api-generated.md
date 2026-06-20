---
## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| questionId | string | query | Нет |  |
| questionIds | array | query | Нет |  |
| urlId | string | query | Нет |  |
| startDate | string | query | Нет |  |
| forceRecalculate | boolean | query | Нет |  |
| minValue | number | query | Нет |  |
| maxValue | number | query | Нет |  |
| limit | number | query | Нет |  |

## Ответ

Возвращает: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## Пример

[inline-code-attrs-start title = 'Пример combine_comments_with_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно; по умолчанию используется https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Ниже приведены примеры для каждого метода аутентификации; используйте тот,
# который соответствует вашему сценарию аутентификации.

# Настройте авторизацию по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если требуется
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (необязательно)
    question_ids = ['question_ids_example'] # List[str] |  (необязательно)
    url_id = 'url_id_example' # str |  (необязательно)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (необязательно)
    force_recalculate = True # bool |  (необязательно)
    min_value = 3.4 # float |  (необязательно)
    max_value = 3.4 # float |  (необязательно)
    limit = 3.4 # float |  (необязательно)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]

---