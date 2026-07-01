## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| state | number | query | לא |  |
| skip | number | query | לא |  |
| limit | number | query | לא |  |

## תגובה

מחזיר: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_tickets'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTicketsOptions
from client.models.get_tickets_response import GetTicketsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ובברירת מחדל https://fastcomments.com
# ראו configuration.py לקבלת רשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמשו בדוגמה שמתאימה
# למקרה השימוש שלכם באימות.

# הגדרת הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטל את ההערה שלמטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (אופציונלי)
    state = 3.4 # float |  (אופציונלי)
    skip = 3.4 # float |  (אופציונלי)
    limit = 3.4 # float |  (אופציונלי)

    try:
        api_response = api_instance.get_tickets(tenant_id, GetTicketsOptions(user_id=user_id, state=state, skip=skip, limit=limit))
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]