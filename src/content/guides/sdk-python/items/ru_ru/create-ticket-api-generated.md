## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |

## Ответ

Возвращает: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_ticket200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример create_ticket'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_ticket200_response import CreateTicket200Response
from client.models.create_ticket_body import CreateTicketBody
from client.rest import ApiException
from pprint import pprint

# Указание параметра host необязательно и по умолчанию равно https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример,
# который соответствует вашему сценарию использования аутентификации.

# Настройте авторизацию по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (e.g. Bearer) для API-ключа при необходимости
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Откройте контекст с экземпляром ApiClient
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса DefaultApi
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    create_ticket_body = client.CreateTicketBody() # CreateTicketBody | 

    try:
        api_response = api_instance.create_ticket(tenant_id, user_id, create_ticket_body)
        print("The response of DefaultApi->create_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_ticket: %s\n" % e)
[inline-code-end]