## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Не |  |
| anonUserId | string | query | Не |  |

## Одговор

Враћа: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment200_response.py)

## Пример

[inline-code-attrs-start title = 'un_flag_comment Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment200_response import FlagComment200Response
from client.rest import ApiException
from pprint import pprint

# Подавање host-а је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора подесити параметре аутентификације и овлашћења
# у складу са безбедносном политиком API сервера.
# Испод су дат примјери за сваки метод аутентификације, користите примјер који
# одговара вашем случају коришћења за аутентификацију.

# Конфигуришите ауторизацију са API кључем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Отворите контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (опционално)
    anon_user_id = 'anon_user_id_example' # str |  (опционално)

    try:
        api_response = api_instance.un_flag_comment(tenant_id, id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->un_flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->un_flag_comment: %s\n" % e)
[inline-code-end]