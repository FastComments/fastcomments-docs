## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## תגובה

Returns: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_moderate_get_user_ban_preferences_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_user_ban_preference'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_moderate_get_user_ban_preferences_response import APIModerateGetUserBanPreferencesResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית והברירת מחדל היא https://fastcomments.com
# ראו קובץ configuration.py לקבלת רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסו הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_user_ban_preference(tenant_id, sso=sso)
        print("התשובה של ModerationApi->get_user_ban_preference:\n")
        pprint(api_response)
    except Exception as e:
        print("שגיאה בעת קריאת ModerationApi->get_user_ban_preference: %s\n" % e)
[inline-code-end]