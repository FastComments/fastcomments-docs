## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_response.py)

## Пример

[inline-code-attrs-start title = 'get_user_badge Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_response import APIGetUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са безбедносном политиком API сервера.
# Испод се налазе примјери за сваки метод аутентификације, користите примјер који
# одговара вашем случају употребе аутентификације.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ако је потребно, уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_user_badge(tenant_id, id)
        print("The response of DefaultApi->get_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge: %s\n" % e)
[inline-code-end]