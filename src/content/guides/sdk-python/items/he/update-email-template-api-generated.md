---
## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_email_template_body import UpdateEmailTemplateBody
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אינה חובה והברירת מחדל היא https://fastcomments.com
# עיין ב-configuration.py לרשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות ניתנות למטה — השתמש בדוגמה שמתאימה למקרה השימוש שלך.

# הגדר הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה מתחת כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_email_template_body = client.UpdateEmailTemplateBody() # UpdateEmailTemplateBody | 

    try:
        api_response = api_instance.update_email_template(tenant_id, id, update_email_template_body)
        print("The response of DefaultApi->update_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_email_template: %s\n" % e)
[inline-code-end]

---