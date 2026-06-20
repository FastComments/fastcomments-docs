## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_templates_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_email_templates'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_templates_response import GetEmailTemplatesResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית וברירת המחדל היא https://fastcomments.com
# עיין ב-configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה שמ
# עונה על מקרה השימוש שלך באימות.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה אם תרצה להגדיר קידומת (למשל Bearer) למפתח ה-API, במידה ונדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (אופציונלי)

    try:
        api_response = api_instance.get_email_templates(tenant_id, skip=skip)
        print("The response of DefaultApi->get_email_templates:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_templates: %s\n" % e)
[inline-code-end]