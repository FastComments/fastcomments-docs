## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| domain | string | path | Да |  |

## Ответ

Возвращает: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_domain_config_response.py)

## Пример

[inline-code-attrs-start title = 'Пример delete_domain_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_domain_config_response import DeleteDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте тот,
# который подходит для вашего случая использования.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (e.g. Bearer) для API key, если необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain = 'domain_example' # str | 

    try:
        api_response = api_instance.delete_domain_config(tenant_id, domain)
        print("The response of DefaultApi->delete_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_domain_config: %s\n" % e)
[inline-code-end]