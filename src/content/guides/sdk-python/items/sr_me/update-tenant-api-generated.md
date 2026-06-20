## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример update_tenant'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_tenant_body import UpdateTenantBody
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са безбедносном политиком API сервера.
# Испод су примери за сваки метод аутентификације, користите пример који
# одговара вашем случају употребе аутентификације.

# Конфигуришите овлашћење преко API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментарајте испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_tenant_body = client.UpdateTenantBody() # UpdateTenantBody | 

    try:
        api_response = api_instance.update_tenant(tenant_id, id, update_tenant_body)
        print("The response of DefaultApi->update_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_tenant: %s\n" % e)
[inline-code-end]