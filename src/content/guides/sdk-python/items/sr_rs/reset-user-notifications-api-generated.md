## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| afterId | string | query | Не |  |
| afterCreatedAt | integer | query | Не |  |
| unreadOnly | boolean | query | Не |  |
| dmOnly | boolean | query | Не |  |
| noDm | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications_response.py)

## Пример

[inline-code-attrs-start title = 'reset_user_notifications Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.reset_user_notifications_response import ResetUserNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумевано на https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (опционо)
    after_created_at = 56 # int |  (опционо)
    unread_only = True # bool |  (опционо)
    dm_only = True # bool |  (опционо)
    no_dm = True # bool |  (опционо)
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.reset_user_notifications(tenant_id, after_id=after_id, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, sso=sso)
        print("The response of PublicApi->reset_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->reset_user_notifications: %s\n" % e)
[inline-code-end]