## פרמטרים

| שם | Type | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| sso | string | query | לא |  |

## Response

מחזיר: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_logs_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_logs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_logs_response import ModerationAPIGetLogsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת host היא אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראו את configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסו להקשר עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_logs(comment_id, sso=sso)
        print("The response of ModerationApi->get_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_logs: %s\n" % e)
[inline-code-end]