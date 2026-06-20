## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| badgeId | string | query | לא |  |
| type | number | query | לא |  |
| displayedOnComments | boolean | query | לא |  |
| limit | number | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראה את configuration.py לרשימת כל פרמטרי הקונפיגורציה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטרי האימות והרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות להלן; השתמש בדוגמה ש
# שמתאימה למקרה השימוש שלך.

# הגדר הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר הערת שורה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# כניסה להקשר עם מופע של הלקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (אופציונלי)
    badge_id = 'badge_id_example' # str |  (אופציונלי)
    type = 3.4 # float |  (אופציונלי)
    displayed_on_comments = True # bool |  (אופציונלי)
    limit = 3.4 # float |  (אופציונלי)
    skip = 3.4 # float |  (אופציונלי)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]