## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_configs_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_question_configs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_configs_response import GetQuestionConfigsResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре аутентификације и овлашћења
# у складу са политиком безбједности API сервера.
# У наставку су наведени примјери за сваку методу аутентификације, користите примјер који
# одговара вашем случају употребе аутентификације.

# Подесите авторизацију помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ако је потребно, уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (необавезно)

    try:
        api_response = api_instance.get_question_configs(tenant_id, skip=skip)
        print("The response of DefaultApi->get_question_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_configs: %s\n" % e)
[inline-code-end]