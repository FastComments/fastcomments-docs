## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Одговор

Враћа: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_configs_response.py)

## Примјер

[inline-code-attrs-start title = 'get_question_configs Примјер'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_configs_response import GetQuestionConfigsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са безбједносном политиком API сервера.
# Примјери за сваки метод аутентификације су дати испод, користите примјер који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите овлашћење помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ако је потребно, уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_question_configs(tenant_id, skip=skip)
        print("The response of DefaultApi->get_question_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_configs: %s\n" % e)
[inline-code-end]