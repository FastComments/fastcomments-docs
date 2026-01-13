## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Response

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Пример

[inline-code-attrs-start title = 'update_tenant_package Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.update_tenant_package_body import UpdateTenantPackageBody
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је на https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре аутентификације и овлашћења
# у складу са политиком безбедности API сервера.
# Испод су примјери за сваки метод аутентификације, користите примјер који
# одговара вашем случају употребе аутентификације.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_tenant_package_body = client.UpdateTenantPackageBody() # UpdateTenantPackageBody | 

    try:
        api_response = api_instance.update_tenant_package(tenant_id, id, update_tenant_package_body)
        print("The response of DefaultApi->update_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_tenant_package: %s\n" % e)
[inline-code-end]