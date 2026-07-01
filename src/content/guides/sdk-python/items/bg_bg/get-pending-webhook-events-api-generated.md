## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | низ | заявка | Да |  |
| commentId | низ | заявка | Не |  |
| externalId | низ | заявка | Не |  |
| eventType | низ | заявка | Не |  |
| type | низ | заявка | Не |  |
| domain | низ | заявка | Не |  |
| attemptCountGT | число | заявка | Не |  |
| skip | число | заявка | Не |  |

## Отговор

Връща: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events_response.py)

## Пример

[inline-code-attrs-start title = 'get_pending_webhook_events Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetPendingWebhookEventsOptions
from client.models.get_pending_webhook_events_response import GetPendingWebhookEventsResponse
from client.rest import ApiException
from pprint import pprint

# Определянето на хоста е опционално и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за автентикация и упълномощаване
# в съответствие с политиката за безопасност на API сървъра.
# Примери за всеки метод за автентикация са предоставени по-долу, използвайте примера, който
# отговаря на вашия сценарий за автентикация.

# Конфигуриране на упълномощаване с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключ, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Въведете контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създаване на инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (по избор)
    external_id = 'external_id_example' # str |  (по избор)
    event_type = 'event_type_example' # str |  (по избор)
    type = 'type_example' # str |  (по избор)
    domain = 'domain_example' # str |  (по избор)
    attempt_count_gt = 3.4 # float |  (по избор)
    skip = 3.4 # float |  (по избор)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, GetPendingWebhookEventsOptions(comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip))
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]