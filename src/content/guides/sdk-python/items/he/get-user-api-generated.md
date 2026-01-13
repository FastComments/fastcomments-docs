## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## תגובה

מחזיר: [`GetUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user200_response import GetUser200Response
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אינה חובה ומוגדרת כברירת מחדל ל־https://fastcomments.com
# ראו את configuration.py לרשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות והרשאות
# בהתאם למדיניות אבטחת שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמשו בדוגמה ש־
# מתאימה למקרה השימוש שלכם באימות.
# הגדר הרשאה באמצעות מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסירו את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה־API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# כנסו לקונטקסט עם מופע של לקוח ה־API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה־API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_user(tenant_id, id)
        print("The response of DefaultApi->get_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user: %s\n" % e)
[inline-code-end]