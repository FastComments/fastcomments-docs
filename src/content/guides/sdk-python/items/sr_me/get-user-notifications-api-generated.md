## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| pageSize | integer | query | Не |  |
| afterId | string | query | Не |  |
| includeContext | boolean | query | Не |  |
| afterCreatedAt | integer | query | Не |  |
| unreadOnly | boolean | query | Не |  |
| dmOnly | boolean | query | Не |  |
| noDm | boolean | query | Не |  |
| includeTranslations | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_notifications200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_user_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_notifications200_response import GetUserNotifications200Response
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Постављање host-а је опционално и подразумева се https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page_size = 56 # int |  (опционо)
    after_id = 'after_id_example' # str |  (опционо)
    include_context = True # bool |  (опционо)
    after_created_at = 56 # int |  (опционо)
    unread_only = True # bool |  (опционо)
    dm_only = True # bool |  (опционо)
    no_dm = True # bool |  (опционо)
    include_translations = True # bool |  (опционо)
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]