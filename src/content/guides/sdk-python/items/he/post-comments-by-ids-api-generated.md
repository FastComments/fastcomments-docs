## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## תגובה

מחזירה: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_child_comments_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה post_comments_by_ids'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comments_by_ids_params import CommentsByIdsParams
from client.models.moderation_api_child_comments_response import ModerationAPIChildCommentsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ומשתמשת כברירת מחדל ב‑https://fastcomments.com
# ראו configuration.py עבור רשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# כניסה להקשר עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה‑API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comments_by_ids_params = client.CommentsByIdsParams() # CommentsByIdsParams | 
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.post_comments_by_ids(tenant_id, comments_by_ids_params, sso=sso)
        print("המענה של ModerationApi->post_comments_by_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("שגיאה בעת קריאה למתודה ModerationApi->post_comments_by_ids: %s\n" % e)
[inline-code-end]