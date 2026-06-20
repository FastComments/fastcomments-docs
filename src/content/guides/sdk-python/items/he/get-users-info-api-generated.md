מידע אצווה על משתמשים עבור טננט. בהינתן userIds, החזר מידע תצוגה מ-User / SSOUser.
משמש על ידי ווידג'ט התגובות להעשיר משתמשים שכרגע הופיעו באמצעות אירוע נוכחות.
אין הקשר של דף: פרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | מזהי משתמשים (userIds) מופרדים בפסיק. |

## תגובה

מחזיר: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_users_info'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ובברירת המחדל היא https://fastcomments.com
# ראה configuration.py לרשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# כניסה להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | מזהי משתמשים (userIds) מופרדים בפסיק.

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]