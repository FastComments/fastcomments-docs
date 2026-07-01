## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |

## תגובה

מחזיר: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notification_count_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetNotificationCountOptions
from client.models.get_notification_count_response import GetNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראו configuration.py לקבלת רשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטי האימות וההרשאה
# בהתאם למדיניות האבטחה של השרת API.
# דוגמאות לכל שיטת אימות מסופקות להלן, השתמשו בדוגמה שמתאימה
# לצרכי האימות שלכם.

# הגדרת הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסירו את ההערה מהשורה הבאה כדי להגדיר קידומת (למשל Bearer) למפתח ה‑API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו לקונטקסט עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה‑API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (אופציונלי)
    url_id = 'url_id_example' # str |  (אופציונלי)
    from_comment_id = 'from_comment_id_example' # str |  (אופציונלי)
    viewed = True # bool |  (אופציונלי)
    type = 'type_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_notification_count(tenant_id, GetNotificationCountOptions(user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type))
        print("The response of DefaultApi->get_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notification_count: %s\n" % e)
[inline-code-end]