## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| urlId | string | query | Ні | Використовується для визначення, чи підписана поточна сторінка. |
| pageSize | integer | query | Ні |  |
| afterId | string | query | Ні |  |
| includeContext | boolean | query | Ні |  |
| afterCreatedAt | integer | query | Ні |  |
| unreadOnly | boolean | query | Ні |  |
| dmOnly | boolean | query | Ні |  |
| noDm | boolean | query | Ні |  |
| includeTranslations | boolean | query | Ні |  |
| includeTenantNotifications | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_user_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням має значення https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Використовується для визначення, чи підписана поточна сторінка. (необов'язково)
    page_size = 56 # int |  (необов'язково)
    after_id = 'after_id_example' # str |  (необов'язково)
    include_context = True # bool |  (необов'язково)
    after_created_at = 56 # int |  (необов'язково)
    unread_only = True # bool |  (необов'язково)
    dm_only = True # bool |  (необов'язково)
    no_dm = True # bool |  (необов'язково)
    include_translations = True # bool |  (необов'язково)
    include_tenant_notifications = True # bool |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]