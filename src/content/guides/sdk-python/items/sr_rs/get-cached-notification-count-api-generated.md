## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_cached_notification_count_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_cached_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_cached_notification_count_response import GetCachedNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подешено је по подразумеваној вредности на https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да подеси аутентификационе и ауторизационе параметре
# у складу са безбедносном политиком API сервера.
# Испод су наведени примери за сваки начин аутентификације, користите пример који
# одговара вашем случају коришћења аутентификације.

# Подешавање овлашћења API кључем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментаришите испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_cached_notification_count(tenant_id, id)
        print("The response of DefaultApi->get_cached_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_cached_notification_count: %s\n" % e)
[inline-code-end]