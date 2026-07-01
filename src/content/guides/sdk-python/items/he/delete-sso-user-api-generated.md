## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## תגובה

מחזיר: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## דוגמה

[inline-code-attrs-start title = 'delete_sso_user דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import DeleteSsoUserOptions
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ומוגדרת כברירת מחדל ל- https://fastcomments.com
# ראו configuration.py עבור רשימת כל פרמטרי ההגדרה הנתמכים.
# על הלקוח להגדיר את פרמטרי האימות והאישור
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמשו בדוגמה ש
# מתאימה למקרה השימוש שלכם באימות.

# הגדרת אימות מפתח API: api_key
# בטלו את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכניסו הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = True # bool |  (אופציונלי)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, DeleteSsoUserOptions(delete_comments=delete_comments, comment_delete_mode=comment_delete_mode))
        print("The response of DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]