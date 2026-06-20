## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| limit | number | query | Не |  |
| skip | number | query | Не |  |
| order | string | query | Не |  |
| after | number | query | Не |  |
| before | number | query | Не |  |

## Одговор

Враћа: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## Пример

[inline-code-attrs-start title = 'get_audit_logs Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумева https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са политиком безбедности API сервера.
# Испод су примери за сваки метод аутентификације, користите пример који
# одговара вашем случају употребе аутентификације.

# Конфигуришите API кључ за ауторизацију: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Унесите контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (необавезно)
    skip = 3.4 # float |  (необавезно)
    order = client.SORTDIR() # SORTDIR |  (необавезно)
    after = 3.4 # float |  (необавезно)
    before = 3.4 # float |  (необавезно)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]