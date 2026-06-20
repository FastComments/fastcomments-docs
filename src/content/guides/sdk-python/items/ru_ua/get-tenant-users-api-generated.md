## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_users_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_tenant_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_users_response import GetTenantUsersResponse
from client.rest import ApiException
from pprint import pprint

# Определение хоста необязательно и по умолчанию https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности API-сервера.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример,
# который соответствует вашему сценарию использования аутентификации.

# Настройка авторизации по ключу API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже для установки префикса (например, Bearer) для ключа API, если требуется
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (необязательно)

    try:
        api_response = api_instance.get_tenant_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_users: %s\n" % e)
[inline-code-end]