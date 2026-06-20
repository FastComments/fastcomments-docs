מגיבים קודמים בדף שאינם מקוונים כרגע. מסודרים לפי displayName.
השתמש בזה לאחר שמיצית את /users/online כדי להציג מדור 'חברים'.
דפדוף מסוג Cursor על commenterName: השרת סורק את האינדקס החלקי {tenantId, urlId, commenterName} החל מ-afterName קדימה באמצעות $gt, ללא עלות של $skip.

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה URL של הדף (מנוקה בצד השרת). |
| afterName | string | query | No | עוגן: העבר את nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | No | שובר תיקו לעוגן: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות זהים לא יאבדו רשומות. |

## תגובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ובברירת המחדל היא https://fastcomments.com
# ראה configuration.py עבור רשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | מזהה URL של הדף (מנוקה בצד השרת).
    after_name = 'after_name_example' # str | עוגן: העבר את nextAfterName מהתגובה הקודמת. (אופציונלי)
    after_user_id = 'after_user_id_example' # str | שובר תיקו לעוגן: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות זהים לא יאבדו רשומות. (אופציונלי)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]