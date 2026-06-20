## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| notificationId | string | path | כן |  |
| newStatus | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_status_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_user_notification_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_status_response import UpdateUserNotificationStatusResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית וברירת המחדל היא https://fastcomments.com
# ראו configuration.py לרשימת כל הפרמטרים הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסו להקשר עם מופע של ApiClient
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    notification_id = 'notification_id_example' # str | 
    new_status = 'new_status_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.update_user_notification_status(tenant_id, notification_id, new_status, sso=sso)
        print("The response of PublicApi->update_user_notification_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_status: %s\n" % e)
[inline-code-end]