## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## תגובה

Returns: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_ban_status_response.py)

## דוגמה

[inline-code-attrs-start title = 'get_comment_ban_status דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_ban_status_response import GetCommentBanStatusResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ובברירת מחדל https://fastcomments.com
# ראה configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_comment_ban_status(tenant_id, comment_id, sso=sso)
        print("The response of ModerationApi->get_comment_ban_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_comment_ban_status: %s\n" % e)
[inline-code-end]