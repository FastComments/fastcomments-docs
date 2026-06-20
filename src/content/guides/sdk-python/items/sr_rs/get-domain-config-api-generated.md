## Параметри

| Име | Тип | Локација | Обавезно | Опис |
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

# Дефинисање host је опционално и подразумева https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и овлашћења
# у складу са политиком безбедности API сервера.
# Испод су наведени примери за сваки метод аутентификације, користите пример који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите API key авторизацију: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте доле да бисте подесили префикс (нпр. Bearer) за API key, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са екземпляром API клијента
with client.ApiClient(configuration) as api_client:
    # Направите екземпляр API класе
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