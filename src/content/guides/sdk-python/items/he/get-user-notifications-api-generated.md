## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | לא | משמש לקביעת האם הדף הנוכחי מנוי. |
| pageSize | integer | query | לא |  |
| afterId | string | query | לא |  |
| includeContext | boolean | query | לא |  |
| afterCreatedAt | integer | query | לא |  |
| unreadOnly | boolean | query | לא |  |
| dmOnly | boolean | query | לא |  |
| noDm | boolean | query | לא |  |
| includeTranslations | boolean | query | לא |  |
| includeTenantNotifications | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית והברירת מחדל היא https://fastcomments.com
# ראו את configuration.py לקבלת רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסו להקשר באמצעות מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | משמש לקביעת האם הדף הנוכחי מנוי. (אופציונלי)
    page_size = 56 # int |  (אופציונלי)
    after_id = 'after_id_example' # str |  (אופציונלי)
    include_context = True # bool |  (אופציונלי)
    after_created_at = 56 # int |  (אופציונלי)
    unread_only = True # bool |  (אופציונלי)
    dm_only = True # bool |  (אופציונלי)
    no_dm = True # bool |  (אופציונלי)
    include_translations = True # bool |  (אופציונלי)
    include_tenant_notifications = True # bool |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]