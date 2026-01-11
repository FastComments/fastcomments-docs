## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| commentId | string | path | כן |  |
| voteId | string | path | כן |  |
| urlId | string | query | כן |  |
| broadcastId | string | query | כן |  |
| editKey | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment_vote200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-delete_comment_vote'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_comment_vote200_response import DeleteCommentVote200Response
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ובברירת מחדל היא https://fastcomments.com
# ראה את configuration.py עבור רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# כניסה להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    vote_id = 'vote_id_example' # str | 
    url_id = 'url_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->delete_comment_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_comment_vote: %s\n" % e)
[inline-code-end]