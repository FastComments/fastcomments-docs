## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Одговор

Враћа: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_configs200_response.py)

## Пример

[inline-code-attrs-start title = 'get_question_configs Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_configs200_response import GetQuestionConfigs200Response
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумева се као https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# у складу са политиком безбедности API сервера.
# Примери за сваки метод аутентификације дати су у наставку; користите онај који
# одговара вашем случају употребе.
#
# Подесити авторизацију API кључем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте испод да бисте подесили префикс (нпр. Bearer) за API key, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (опционо)

    try:
        api_response = api_instance.get_question_configs(tenant_id, skip=skip)
        print("The response of DefaultApi->get_question_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_configs: %s\n" % e)
[inline-code-end]