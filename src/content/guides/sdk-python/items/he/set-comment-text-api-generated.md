## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| commentId | string | path | כן |  |
| broadcastId | string | query | כן |  |
| editKey | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_set_comment_text_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת set_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import SetCommentTextOptions
from client.models.comment_text_update_request import CommentTextUpdateRequest
from client.models.public_api_set_comment_text_response import PublicAPISetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ומשמשת כברירת מחדל https://fastcomments.com
# ראו configuration.py לקבלת רשימת כל פרמטרי הקונפיגורציה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסו להקשר עם מופע של לקוח ה‑API
# צרו מופע של מחלקת ה‑API
with client.ApiClient(configuration) as api_client:
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str |
    comment_id = 'comment_id_example' # str |
    broadcast_id = 'broadcast_id_example' # str |
    comment_text_update_request = client.CommentTextUpdateRequest() # CommentTextUpdateRequest |
    edit_key = 'edit_key_example' # str |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, SetCommentTextOptions(edit_key=edit_key, sso=sso))
        print("The response of PublicApi->set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->set_comment_text: %s\n" % e)
[inline-code-end]