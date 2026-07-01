## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| direction | string | query | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## תשובה

מחזיר: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## דוגמה

[inline-code-attrs-start title = 'create_vote דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import CreateVoteOptions
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ובברירת המחדל https://fastcomments.com
# ראה configuration.py לקבלת רשימה של כל פרמטרי הקונפיגורציה הנתמכים.
# הלקוח חייב להגדיר את פרמטי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות ניתנות למטה, השתמש בדוגמה שמ
# מתאימה למקרה השימוש שלך באימות.
# הגדר הרשאת מפתח API: api_key
# בטל את ההערה של הקו הבא כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
# configuration.api_key['api_key'] = os.environ["API_KEY"]
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str | 
    user_id = 'user_id_example' # str |  (אופציונלי)
    anon_user_id = 'anon_user_id_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.create_vote(tenant_id, comment_id, direction, CreateVoteOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("The response of DefaultApi->create_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_vote: %s\n" % e)
[inline-code-end]