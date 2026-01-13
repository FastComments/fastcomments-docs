## Parameters

| Имя | Тип | Location | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Response

Возвращает: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_sso_user_api_response.py)

## Пример

[inline-code-attrs-start title = 'Пример add_sso_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_sso_user_api_response import AddSSOUserAPIResponse
from client.models.create_apisso_user_data import CreateAPISSOUserData
from client.rest import ApiException
from pprint import pprint

# Определение host необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Ниже приведены примеры для каждого метода аутентификации, используйте пример, который
# соответствует вашему сценарию аутентификации.

# Настройка авторизации по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (e.g. Bearer) для API-ключа, если это необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_apisso_user_data = client.CreateAPISSOUserData() # CreateAPISSOUserData | 

    try:
        api_response = api_instance.add_sso_user(tenant_id, create_apisso_user_data)
        print("The response of DefaultApi->add_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_sso_user: %s\n" % e)
[inline-code-end]