## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| contextUserId | string | query | לא |  |
| isLive | boolean | query | לא |  |

## תגובה

מחזיר: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת delete_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_comment200_response import DeleteComment200Response
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית והברירת מחדל היא https://fastcomments.com
# ראה את configuration.py לרשימה של כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח צריך להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה ש
# מספקת את מקרה השימוש שלך.
 
# קבע הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם צריך
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס הקשר עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    context_user_id = 'context_user_id_example' # str |  (אופציונלי)
    is_live = True # bool |  (אופציונלי)

    try:
        api_response = api_instance.delete_comment(tenant_id, id, context_user_id=context_user_id, is_live=is_live)
        print("The response of DefaultApi->delete_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_comment: %s\n" % e)
[inline-code-end]