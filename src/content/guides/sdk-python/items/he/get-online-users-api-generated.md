כעת‑במקוון של צופים בדף: אנשים שה‑websocket שלהם מנוי לדף ברגע זה.  
מחזיר `anonCount + totalCount` (מנויים בכל החדר, כולל צופים אנונימיים שלא אנו מונים).

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן | מזהה כתובת URL של הדף (נוקה בצד השרת). |
| afterName | string | query | לא | סמן: העבר `nextAfterName` מהתגובה הקודמת. |
| afterUserId | string | query | לא | פתרון קודקוד של סמן: העבר `nextAfterUserId` מהתגובה הקודמת. נדרש כאשר `afterName` מוגדר כדי שמזגי שמות לא יופלו. |

## תשובה

מחזיר: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## דוגמה

[inline-code-attrs-start title = 'get_online_users דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה‑host היא אופציונלית ומוגדרת כברירת מחדל ל‑https://fastcomments.com
# ראה קובץ configuration.py לקבלת רשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנס קונטקסט עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה‑API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | מזהה כתובת URL של הדף (נוקה בצד השרת).
    after_name = 'after_name_example' # str | סמן: העבר nextAfterName מהתגובה הקודמת. (אופציונלי)
    after_user_id = 'after_user_id_example' # str | פתרון קודקוד של סמן: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמזגי שמות לא יופלו. (אופציונלי)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]