## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_badge_progress_by_id'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_response import APIGetUserBadgeProgressResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ובברירת מחדל היא https://fastcomments.com
# עיין ב-configuration.py לרשימת כל פרמטרי ההגדרות הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות והרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה ש
# מתאימה למקרה השימוש שלך.

# הגדר הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# היכנס להקשר עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_user_badge_progress_by_id(tenant_id, id)
        print("The response of DefaultApi->get_user_badge_progress_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_by_id: %s\n" % e)
[inline-code-end]