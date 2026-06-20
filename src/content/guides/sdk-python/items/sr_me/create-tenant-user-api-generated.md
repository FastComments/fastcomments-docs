## Parameters

| Name | Type | Location | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Response

Враћа: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_user_response.py)

## Пример

[inline-code-attrs-start title = 'create_tenant_user Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_user_body import CreateTenantUserBody
from client.models.create_tenant_user_response import CreateTenantUserResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционо и подразумева се https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре аутентификације и овлашћења
# у складу са политиком безбедности API сервера.
# Примери за сваки метод аутентификације дати су доле, користите пример који
# одговара вашем случају употребе аутентификације.

# Конфигуришите овлашћење помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментаришите испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_user_body = client.CreateTenantUserBody() # CreateTenantUserBody | 

    try:
        api_response = api_instance.create_tenant_user(tenant_id, create_tenant_user_body)
        print("The response of DefaultApi->create_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_user: %s\n" % e)
[inline-code-end]