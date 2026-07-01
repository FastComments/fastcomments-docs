## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| limit | number | query | Нет |  |
| skip | number | query | Нет |  |
| order | string | query | Нет |  |
| after | number | query | Нет |  |
| before | number | query | Нет |  |

## Ответ

Возвращает: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## Пример

[inline-code-attrs-start title = 'get_audit_logs Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetAuditLogsOptions
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Определение хоста является необязательным и по умолчанию https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности API сервера.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример, который
# удовлетворяет вашему случаю использования аутентификации.

# Настройка авторизации ключом API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float | (необязательно)
    skip = 3.4 # float | (необязательно)
    order = client.SORTDIR() # SORTDIR | (необязательно)
    after = 3.4 # float | (необязательно)
    before = 3.4 # float | (необязательно)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, GetAuditLogsOptions(limit=limit, skip=skip, order=order, after=after, before=before))
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]