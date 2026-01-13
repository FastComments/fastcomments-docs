## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_subscription_api_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת create_subscription'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_api_user_subscription_data import CreateAPIUserSubscriptionData
from client.models.create_subscription_api_response import CreateSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית והערך המחדלי הוא https://fastcomments.com
# ראו configuration.py לרשימת כל הפרמטרים הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות והרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות להלן; השתמשו בדוגמה שמתאימה למקרה השימוש שלכם.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסירו את ההערה מתחת כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם צריך
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו לקונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_api_user_subscription_data = client.CreateAPIUserSubscriptionData() # CreateAPIUserSubscriptionData | 

    try:
        api_response = api_instance.create_subscription(tenant_id, create_api_user_subscription_data)
        print("The response of DefaultApi->create_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_subscription: %s\n" % e)
[inline-code-end]