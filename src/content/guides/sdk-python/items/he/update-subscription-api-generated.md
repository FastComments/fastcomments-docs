## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| userId | string | query | לא |  |

## תגובה

מחזיר: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_subscription_api_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_subscription'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_api_user_subscription_data import UpdateAPIUserSubscriptionData
from client.models.update_subscription_api_response import UpdateSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית וערך ברירת המחדל הוא https://fastcomments.com
# עיין בקובץ configuration.py עבור רשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות אבטחת שרת ה-API.
# דוגמאות לכל שיטת אימות מופיעות למטה; השתמש בדוגמה המתאימה למקרה השימוש שלך.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר (context) עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_api_user_subscription_data = client.UpdateAPIUserSubscriptionData() # UpdateAPIUserSubscriptionData | 
    user_id = 'user_id_example' # str |  (optional)

    try:
        api_response = api_instance.update_subscription(tenant_id, id, update_api_user_subscription_data, user_id=user_id)
        print("The response of DefaultApi->update_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_subscription: %s\n" % e)
[inline-code-end]