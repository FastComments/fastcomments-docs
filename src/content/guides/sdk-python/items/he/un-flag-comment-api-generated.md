## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## תגובה

מחזיר: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת un_flag_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import UnFlagCommentOptions
from client.models.flag_comment_response import FlagCommentResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ומוגדרת כברירת מחדל ל‑https://fastcomments.com
# ראה configuration.py עבור רשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב לקבוע את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה‑API.
# דוגמאות לכל שיטת אימות ניתנות למטה, השתמשו בדוגמה שהיא
# מתאימה למקרה השימוש שלכם באימות.

# הגדר הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטלו את ההערה של הקוד להלן כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו לקונטקסט עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה‑API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)

    try:
        api_response = api_instance.un_flag_comment(tenant_id, id, UnFlagCommentOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("The response of DefaultApi->un_flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->un_flag_comment: %s\n" % e)
[inline-code-end]

---