צופים המחוברים כעת לעמוד: אנשים שסשן ה-websocket שלהם מנוי לעמוד כרגע.
מחזיר anonCount + totalCount (מנויים בחדר כולו, כולל צופים אנונימיים שאותם איננו מפורטים).

## פרמטרים

| Name | Type | Location | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן | מזהה URL של העמוד (נוקה בצד השרת). |
| afterName | string | query | לא | סמן (Cursor): העבר את nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | לא | מפריד תיקו של ה-cursor: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות זהים לא יגרמו לאיבוד רשומות. |

## תגובה

מחזיר: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה עבור get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית וברירת המחדל היא https://fastcomments.com
# ראה את configuration.py לרשימת כל פרמטרי הקונפיגורציה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | מזהה URL של העמוד (נוקה בצד השרת).
    after_name = 'after_name_example' # str | סמן (Cursor): העבר את nextAfterName מהתגובה הקודמת. (אופציונלי)
    after_user_id = 'after_user_id_example' # str | מפריד תיקו של ה-cursor: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות זהים לא יגרמו לאיבוד רשומות. (אופציונלי)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]