## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## תגובה

מחזיר: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_badge200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_user_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_badge200_response import GetUserBadge200Response
from client.rest import ApiException
from pprint import pprint

# הגדרת host אינה חובה והברירת מחדל היא https://fastcomments.com
# ראו את configuration.py עבור רשימת כל פרמטרי הקונפיגורציה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב לקבוע את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מוצגות למטה, השתמש בדוגמה ש
# שעונה על מקרה השימוש של האימות שלך.

# קונפיגורציית הרשאת מפתח ה-API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_user_badge(tenant_id, id)
        print("The response of DefaultApi->get_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge: %s\n" % e)
[inline-code-end]