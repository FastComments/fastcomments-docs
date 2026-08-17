## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| afterId | string | query | Не |  |
| afterCreatedAt | integer | query | Не |  |
| unreadOnly | boolean | query | Не |  |
| dmOnly | boolean | query | Не |  |
| noDm | boolean | query | Не |  |
| sso | string | query | Не |  |

## Response

Враћа: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications_response.py)

## Example

[inline-code-attrs-start title = 'reset_user_notifications Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import ResetUserNotificationsOptions
from client.models.reset_user_notifications_response import ResetUserNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционо и подразумева https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (опционално)
    after_created_at = 56 # int |  (опционално)
    unread_only = True # bool |  (опционално)
    dm_only = True # bool |  (опционално)
    no_dm = True # bool |  (опционално)
    sso = 'sso_example' # str |  (опционално)

    try:
        api_response = api_instance.reset_user_notifications(tenant_id, ResetUserNotificationsOptions(after_id=after_id, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, sso=sso))
        print("Одговор PublicApi->reset_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Изузетак при позиву PublicApi->reset_user_notifications: %s\n" % e)
[inline-code-end]