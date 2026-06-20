## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_text_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_moderation_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_text_response import GetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית ומוגדרת כברירת מחדל ל https://fastcomments.com
# ראה את configuration.py לקבלת רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# כניסה להקשר עם מופע של ApiClient
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_moderation_comment_text(comment_id, sso=sso)
        print("The response of ModerationApi->get_moderation_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_moderation_comment_text: %s\n" % e)
[inline-code-end]

---