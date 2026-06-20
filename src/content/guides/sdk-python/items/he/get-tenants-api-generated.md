## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| meta | string | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_tenants'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenants_response import GetTenantsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית והיא ברירת המחדל היא https://fastcomments.com
# עיין בקובץ configuration.py לקבלת רשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח צריך להגדיר את פרמטרי האימות והרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה — השתמש בדוגמה שמתאימה למקרה השימוש שלך.

# הגדר אישור מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם צריך
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (אופציונלי)
    skip = 3.4 # float |  (אופציונלי)

    try:
        api_response = api_instance.get_tenants(tenant_id, meta=meta, skip=skip)
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]