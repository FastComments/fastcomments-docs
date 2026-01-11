## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`GetComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment200_response import GetComment200Response
from client.rest import ApiException
from pprint import pprint

# הגדרת host אופציונלית והברירת מחדל היא https://fastcomments.com
# ראה את configuration.py לרשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על ה-client להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה — השתמש בדוגמה שמתאימה למקרה השימוש שלך.

# קבע הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי לקבוע קידומת (למשל Bearer) עבור מפתח ה-API, אם צריך
# configuration.api_key_prefix['api_key'] = 'Bearer'

# פתח הקשר עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_comment(tenant_id, id)
        print("The response of DefaultApi->get_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comment: %s\n" % e)
[inline-code-end]