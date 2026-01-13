## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | כן |  |

## תגובה

מחזיר: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_votes'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes200_response import GetVotes200Response
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית והברירת מחדל היא https://fastcomments.com
# עיין ב-configuration.py כדי לראות רשימה של כל פרמטרי הקונפיגורציה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה שמתאימה למקרה השימוש שלך.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם צריך
# configuration.api_key_prefix['api_key'] = 'Bearer'

# פתח הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_votes(tenant_id, url_id)
        print("The response of DefaultApi->get_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes: %s\n" % e)
[inline-code-end]