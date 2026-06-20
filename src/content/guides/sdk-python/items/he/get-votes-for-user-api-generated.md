## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | כן |  |
| userId | string | query | לא |  |
| anonUserId | string | query | לא |  |

## תגובה

מחזיר: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_for_user_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_votes_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes_for_user_response import GetVotesForUserResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית וברירת המחדל היא https://fastcomments.com
# ראה configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה — השתמש בדוגמה
# שמתאימה למקרה השימוש שלך.

# הגדר אימות באמצעות מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה להלן כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם יש צורך
# configuration.api_key_prefix['api_key'] = 'Bearer'

# היכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id = 'user_id_example' # str |  (אופציונלי)
    anon_user_id = 'anon_user_id_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_votes_for_user(tenant_id, url_id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->get_votes_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes_for_user: %s\n" % e)
[inline-code-end]