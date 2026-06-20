---
## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| domain | string | path | Да |  |

## Одговор

Враћа: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_domain_config_response.py)

## Пример

[inline-code-attrs-start title = 'get_domain_config Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_domain_config_response import GetDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# Постављање хоста је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и овлашћења
# у складу са безбједносном политиком API сервера.
# Испод су примјери за сваки метод аутентификације, користите примјер који
# одговара вашем случају употребе аутентификације.

# Конфигуришите API key овлашћење: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain = 'domain_example' # str | 

    try:
        api_response = api_instance.get_domain_config(tenant_id, domain)
        print("The response of DefaultApi->get_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_domain_config: %s\n" % e)
[inline-code-end]

---