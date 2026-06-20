---
## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_response import GetEmailTemplateResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית וברירת המחדל היא https://fastcomments.com
# ראה configuration.py לרשימת כל הפרמטרים הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטרי האימות וההרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה
# שמתאימה למקרה השימוש שלך.

# הגדר הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# פתח הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_email_template(tenant_id, id)
        print("The response of DefaultApi->get_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template: %s\n" % e)
[inline-code-end]

---