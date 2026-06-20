## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Нет |  |
| externalId | string | query | Нет |  |
| eventType | string | query | Нет |  |
| type | string | query | Нет |  |
| domain | string | query | Нет |  |
| attemptCountGT | number | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_pending_webhook_events'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_events_response import GetPendingWebhookEventsResponse
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Ниже приведены примеры для каждого метода аутентификации, используйте пример,
# который соответствует вашему сценарию использования аутентификации.

# Настройка авторизации с помощью API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (e.g. Bearer) для API-ключа, если требуется
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (необязательно)
    external_id = 'external_id_example' # str |  (необязательно)
    event_type = 'event_type_example' # str |  (необязательно)
    type = 'type_example' # str |  (необязательно)
    domain = 'domain_example' # str |  (необязательно)
    attempt_count_gt = 3.4 # float |  (необязательно)
    skip = 3.4 # float |  (необязательно)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip)
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]