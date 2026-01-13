## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## תגובה

מחזיר: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_cached_notification_count200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_cached_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_cached_notification_count200_response import GetCachedNotificationCount200Response
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# עיין ב-configuration.py לקבלת רשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה; השתמש בדוגמה שמתאימה למקרה השימוש שלך.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# פתח קונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_cached_notification_count(tenant_id, id)
        print("The response of DefaultApi->get_cached_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_cached_notification_count: %s\n" % e)
[inline-code-end]