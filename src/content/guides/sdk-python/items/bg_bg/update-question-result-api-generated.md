## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'update_question_result Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_question_result_body import UpdateQuestionResultBody
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за автентикация и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# По-долу са дадени примери за всеки метод на автентикация, използвайте примера който
# отговаря на вашия сценарий на използване.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_question_result_body = client.UpdateQuestionResultBody() # UpdateQuestionResultBody | 

    try:
        api_response = api_instance.update_question_result(tenant_id, id, update_question_result_body)
        print("The response of DefaultApi->update_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_question_result: %s\n" % e)
[inline-code-end]