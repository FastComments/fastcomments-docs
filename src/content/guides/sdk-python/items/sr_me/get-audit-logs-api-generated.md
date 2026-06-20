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

# Поставка хоста је опционална и подразумева се https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре аутентификације и овлашћења
# у складу са политиком безбедности API сервера.
# Испод су примери за сваки метод аутентификације, користите пример који
# одговара вашем случају употребе.

# Конфигуришите овлашћење помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)
    order = client.SORTDIR() # SORTDIR |  (optional)
    after = 3.4 # float |  (optional)
    before = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]