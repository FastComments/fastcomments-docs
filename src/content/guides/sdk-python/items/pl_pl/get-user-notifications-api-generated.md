## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | Używany do określenia, czy bieżąca strona jest subskrybowana. |
| pageSize | integer | query | No |  |
| afterId | string | query | No |  |
| includeContext | boolean | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| includeTranslations | boolean | query | No |  |
| includeTenantNotifications | boolean | query | No |  |
| sso | string | query | No |  |

## Odpowiedź

Zwraca: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## Przykład

[inline-code-attrs-start title = 'get_user_notifications Przykład'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetUserNotificationsOptions
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Definiowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracyjnych.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Używany do określenia, czy bieżąca strona jest subskrybowana. (opcjonalnie)
    page_size = 56 # int |  (opcjonalnie)
    after_id = 'after_id_example' # str |  (opcjonalnie)
    include_context = True # bool |  (opcjonalnie)
    after_created_at = 56 # int |  (opcjonalnie)
    unread_only = True # bool |  (opcjonalnie)
    dm_only = True # bool |  (opcjonalnie)
    no_dm = True # bool |  (opcjonalnie)
    include_translations = True # bool |  (opcjonalnie)
    include_tenant_notifications = True # bool |  (opcjonalnie)
    sso = 'sso_example' # str |  (opcjonalnie)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, GetUserNotificationsOptions(url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso))
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]