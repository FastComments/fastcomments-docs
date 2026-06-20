## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | כן |  |
| userId | string | query | לא |  |
| commentId | string | query | לא |  |
| broadcastId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/award_user_badge_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-put_award_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.award_user_badge_response import AwardUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראה את configuration.py לרשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסו לקונטקסט עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (אופציונלי)
    comment_id = 'comment_id_example' # str |  (אופציונלי)
    broadcast_id = 'broadcast_id_example' # str |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.put_award_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_award_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_award_badge: %s\n" % e)
[inline-code-end]