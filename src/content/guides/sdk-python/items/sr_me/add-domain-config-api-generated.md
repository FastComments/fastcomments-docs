## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_domain_config_response.py)

## Пример

[inline-code-attrs-start title = 'add_domain_config Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_domain_config_params import AddDomainConfigParams
from client.models.add_domain_config_response import AddDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да подеси параметре аутентификације и ауторизације
# у складу са безбедносном политиком API сервера.
# Примери за сваки метод аутентификације дати су испод, користите пример који
# одговара вашем случају коришћења аутентификације.

# Подешавање ауторизације помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментаришите испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    add_domain_config_params = client.AddDomainConfigParams() # AddDomainConfigParams | 

    try:
        api_response = api_instance.add_domain_config(tenant_id, add_domain_config_params)
        print("The response of DefaultApi->add_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_domain_config: %s\n" % e)
[inline-code-end]