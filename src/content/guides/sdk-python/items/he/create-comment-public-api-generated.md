## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| broadcastId | string | query | Yes |  |
| sessionId | string | query | No |  |
| sso | string | query | No |  |

## תגובה

מחזיר: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comments_response_with_presence.py)

## דוגמה

[inline-code-attrs-start title = 'create_comment_public דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import CreateCommentPublicOptions
from client.models.comment_data import CommentData
from client.models.save_comments_response_with_presence import SaveCommentsResponseWithPresence
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראה configuration.py עבור רשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנס לקונטקסט עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    comment_data = client.CommentData() # CommentData | 
    session_id = 'session_id_example' # str |  (לא חובה)
    sso = 'sso_example' # str |  (לא חובה)

    try:
        api_response = api_instance.create_comment_public(tenant_id, url_id, broadcast_id, comment_data, CreateCommentPublicOptions(session_id=session_id, sso=sso))
        print("The response of PublicApi->create_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_comment_public: %s\n" % e)
[inline-code-end]

---