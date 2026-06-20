## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_config_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_question_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_config_response import GetQuestionConfigResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אינה חובה וברירת המחדל היא https://fastcomments.com
# עיין בקובץ configuration.py לרשימת כל פרמטרי הקונפיגורציה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות והרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה; השתמש בדוגמה ש
# מתאימה למקרה השימוש שלך.

# הגדר אימות באמצעות מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר (context) עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_config(tenant_id, id)
        print("The response of DefaultApi->get_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_config: %s\n" % e)
[inline-code-end]

---