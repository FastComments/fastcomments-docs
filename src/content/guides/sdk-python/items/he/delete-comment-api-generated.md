## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| contextUserId | string | query | לא |  |
| isLive | boolean | query | לא |  |

## תגובה

מחזירה: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment_result.py)

## דוגמה

[inline-code-attrs-start title = 'delete_comment דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import DeleteCommentOptions
from client.models.delete_comment_result import DeleteCommentResult
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ומוגדרת כברירת מחדל ל‑https://fastcomments.com
# ראו configuration.py לקבלת רשימת כל פרמטרי התצורה הנתמכים.
# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה‑API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמשו בדוגמא שתואמת
# את משימת האימות שלכם.

# הגדרת הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה‑API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# תחלקו הקשר עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה‑API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    context_user_id = 'context_user_id_example' # str |  (optional)
    is_live = True # bool |  (optional)

    try:
        api_response = api_instance.delete_comment(tenant_id, id, DeleteCommentOptions(context_user_id=context_user_id, is_live=is_live))
        print("The response of DefaultApi->delete_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_comment: %s\n" % e)
[inline-code-end]