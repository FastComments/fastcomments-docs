## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/change_ticket_state_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת change_ticket_state'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.change_ticket_state_body import ChangeTicketStateBody
from client.models.change_ticket_state_response import ChangeTicketStateResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית והברירת מחדל היא https://fastcomments.com
# עיין ב-configuration.py עבור רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות להלן, השתמש בדוגמה ש-
# מתאימה למקרה השימוש שלך באימות.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה משורה להלן כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נחוץ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס לקונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    id = 'id_example' # str | 
    change_ticket_state_body = client.ChangeTicketStateBody() # ChangeTicketStateBody | 

    try:
        api_response = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
        print("The response of DefaultApi->change_ticket_state:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->change_ticket_state: %s\n" % e)
[inline-code-end]