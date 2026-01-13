## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_package200_response.py)

## Пример

[inline-code-attrs-start title = 'create_tenant_package Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_package200_response import CreateTenantPackage200Response
from client.models.create_tenant_package_body import CreateTenantPackageBody
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са политиком безбедности API сервера.
# Примери за сваки метод аутентификације су наведени испод, користите пример који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите ауторизацију API кључем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментаришите доле да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_package_body = client.CreateTenantPackageBody() # CreateTenantPackageBody | 

    try:
        api_response = api_instance.create_tenant_package(tenant_id, create_tenant_package_body)
        print("The response of DefaultApi->create_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_package: %s\n" % e)
[inline-code-end]