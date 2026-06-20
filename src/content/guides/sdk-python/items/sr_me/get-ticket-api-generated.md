## Параметри

| Name | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Не |  |

## Одговор

Враћа: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_ticket_response.py)

## Пример

[inline-code-attrs-start title = 'get_ticket Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_ticket_response import GetTicketResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и по дефаулту је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентикације и овлашћења
# у складу са безбедносном политиком API сервера.
# Примери за сваки метод аутентикације су дати испод, користите пример који
# одговара вашем случају употребе аутентикације.

# Конфигуришите ауторизацију помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (опционо)

    try:
        api_response = api_instance.get_ticket(tenant_id, id, user_id=user_id)
        print("The response of DefaultApi->get_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_ticket: %s\n" % e)
[inline-code-end]