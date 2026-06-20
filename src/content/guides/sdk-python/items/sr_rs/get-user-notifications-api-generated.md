## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Не | Користи се за утврђивање да ли је тренутна страница претплаћена. |
| pageSize | integer | query | Не |  |
| afterId | string | query | Не |  |
| includeContext | boolean | query | Не |  |
| afterCreatedAt | integer | query | Не |  |
| unreadOnly | boolean | query | Не |  |
| dmOnly | boolean | query | Не |  |
| noDm | boolean | query | Не |  |
| includeTranslations | boolean | query | Не |  |
| includeTenantNotifications | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## Пример

[inline-code-attrs-start title = 'get_user_notifications пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционо и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Отворите контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Направите инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Користи се да би се утврдило да ли је тренутна страница претплаћена. (опционо)
    page_size = 56 # int |  (опционо)
    after_id = 'after_id_example' # str |  (опционо)
    include_context = True # bool |  (опционо)
    after_created_at = 56 # int |  (опционо)
    unread_only = True # bool |  (опционо)
    dm_only = True # bool |  (опционо)
    no_dm = True # bool |  (опционо)
    include_translations = True # bool |  (опционо)
    include_tenant_notifications = True # bool |  (опционо)
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]