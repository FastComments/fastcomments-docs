## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Не |  |
| externalId | string | query | Не |  |
| eventType | string | query | Не |  |
| type | string | query | Не |  |
| domain | string | query | Не |  |
| attemptCountGT | number | query | Не |  |

## Одговор

Враћа: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_pending_webhook_event_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_event_count_response import GetPendingWebhookEventCountResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да подеси параметре аутентификације и ауторизације
# у складу са политиком безбедности API сервера.
# Испод су примери за сваку методу аутентификације, користите пример који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите ауторизацију помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст користећи инстанцу API клијента
with client.ApiClient(configuration) as api_client:
    # Направите инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (опционо)
    external_id = 'external_id_example' # str |  (опционо)
    event_type = 'event_type_example' # str |  (опционо)
    type = 'type_example' # str |  (опционо)
    domain = 'domain_example' # str |  (опционо)
    attempt_count_gt = 3.4 # float |  (опционо)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt)
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]