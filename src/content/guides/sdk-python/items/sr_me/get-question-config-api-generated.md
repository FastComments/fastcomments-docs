## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Одговор

Враћа: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_config_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_question_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_config_response import GetQuestionConfigResponse
from client.rest import ApiException
from pprint import pprint

# Подразумевани host је https://fastcomments.com; његово дефинисање је опционално
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да подеси параметре аутентификације и ауторизације
# у складу са безбедносном политиком API сервера.
# Ниже су дати примери за сваки метод аутентификације; користите онај пример који
# одговара вашем случају коришћења аутентификације.

# Подешавање ауторизације помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте доле да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст помоћу инстанце API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_config(tenant_id, id)
        print("The response of DefaultApi->get_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_config: %s\n" % e)
[inline-code-end]