## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderator_response.py)

## Пример

[inline-code-attrs-start title = 'get_moderator Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderator_response import GetModeratorResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционо и подразумева се https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са безбедносном политиком API сервера.
# Испод су примери за сваки метод аутентификације; користите онај који
# одговара вашем случају употребе.
# Конфигуришите овлашћење API кључем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ако је потребно, откоментирајте доле да бисте поставили префикс (нпр. Bearer) за API кључ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_moderator(tenant_id, id)
        print("The response of DefaultApi->get_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderator: %s\n" % e)
[inline-code-end]