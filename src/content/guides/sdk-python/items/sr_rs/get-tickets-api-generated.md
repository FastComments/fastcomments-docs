## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| state | number | query | Не |  |
| skip | number | query | Не |  |
| limit | number | query | Не |  |

## Одговор

Враћа: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_tickets'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets_response import GetTicketsResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и овлашћења
# у складу са сигурносном политиком API сервера.
# Испод су примери за сваки метод аутентификације, употребите пример који
# одговара вашем случају употребе аутентификације.

# Конфигуришите аутентификацију помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Уклоните коментар испод да бисте поставили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (опционо)
    state = 3.4 # float |  (опционо)
    skip = 3.4 # float |  (опционо)
    limit = 3.4 # float |  (опционо)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]