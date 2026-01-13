## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Ответ

Возвращает: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример create_tenant'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant200_response import CreateTenant200Response
from client.models.create_tenant_body import CreateTenantBody
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже; используйте пример, который
# соответствует вашему сценарию использования аутентификации.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_body = client.CreateTenantBody() # CreateTenantBody | 

    try:
        api_response = api_instance.create_tenant(tenant_id, create_tenant_body)
        print("The response of DefaultApi->create_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant: %s\n" % e)
[inline-code-end]