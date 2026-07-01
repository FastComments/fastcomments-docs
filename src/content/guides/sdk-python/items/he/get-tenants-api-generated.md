## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## Response

מחזיר: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_tenants'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTenantsOptions
from client.models.get_tenants_response import GetTenantsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח הוא אופציונלי והערך ברירת המחדל הוא https://fastcomments.com
# ראה configuration.py לקבלת רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה‑API.
# דוגמאות לכל שיטת אימות מוצגות למטה, השתמשו בדוגמה המתאימה למקרה השימוש שלכם.

# הגדרת אימות מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטלו את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) למפתח ה‑API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו להקשר עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה‑API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (אופציונלי)
    skip = 3.4 # float |  (אופציונלי)

    try:
        api_response = api_instance.get_tenants(tenant_id, GetTenantsOptions(meta=meta, skip=skip))
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]