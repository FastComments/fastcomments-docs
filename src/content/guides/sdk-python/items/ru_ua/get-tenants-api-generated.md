## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| meta | string | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_tenants'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenants_response import GetTenantsResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример,
# который соответствует вашему случаю использования аутентификации.

# Настройка авторизации по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (e.g. Bearer) для API-ключа, если необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр API-класса
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (необязательно)
    skip = 3.4 # float |  (необязательно)

    try:
        api_response = api_instance.get_tenants(tenant_id, meta=meta, skip=skip)
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]