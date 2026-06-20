## Параметри

| Име | Тип | Локација | Захтевано | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_result_response.py)

## Пример

[inline-code-attrs-start title = 'Пример create_question_result'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_result_body import CreateQuestionResultBody
from client.models.create_question_result_response import CreateQuestionResultResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и овлашћивања
# у складу са политиком безбедности API сервера.
# Примери за сваки метод аутентификације су наведени у наставку, користите пример који
# одговара вашем случају употребе аутентификације.

# Конфигуришите API key ауторизацију: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментаришите доле да бисте подесили префикс (нпр. Bearer) за API key, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_question_result_body = client.CreateQuestionResultBody() # CreateQuestionResultBody | 

    try:
        api_response = api_instance.create_question_result(tenant_id, create_question_result_body)
        print("The response of DefaultApi->create_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_question_result: %s\n" % e)
[inline-code-end]