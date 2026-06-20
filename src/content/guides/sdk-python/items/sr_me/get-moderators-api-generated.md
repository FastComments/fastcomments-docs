## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderators_response.py)

## Пример

[inline-code-attrs-start title = 'get_moderators Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderators_response import GetModeratorsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са безбедносном политиком API сервера.
# Испод су дати примјери за сваки метод аутентификације, користите примјер који
# одговара вашем случају употребе аутентификације.

# Конфигуришите авторизацију помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте доле да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_moderators(tenant_id, skip=skip)
        print("The response of DefaultApi->get_moderators:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderators: %s\n" % e)
[inline-code-end]