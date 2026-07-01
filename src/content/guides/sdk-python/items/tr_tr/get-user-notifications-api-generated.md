## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | Mevcut sayfanın abone olup olmadığını belirlemek için kullanılır. |
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

## Yanıt

Returns: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## Örnek

[inline-code-attrs-start title = 'get_user_notifications Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetUserNotificationsOptions
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine sahiptir
# configuration.py dosyasında desteklenen tüm konfigürasyon parametrelerinin listesini görebilirsiniz.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisi örneğiyle bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Mevcut sayfanın abone olup olmadığını belirlemek için kullanılır. (isteğe bağlı)
    page_size = 56 # int |  (isteğe bağlı)
    after_id = 'after_id_example' # str |  (isteğe bağlı)
    include_context = True # bool |  (isteğe bağlı)
    after_created_at = 56 # int |  (isteğe bağlı)
    unread_only = True # bool |  (isteğe bağlı)
    dm_only = True # bool |  (isteğe bağlı)
    no_dm = True # bool |  (isteğe bağlı)
    include_translations = True # bool |  (isteğe bağlı)
    include_tenant_notifications = True # bool |  (isteğe bağlı)
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, GetUserNotificationsOptions(url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso))
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]