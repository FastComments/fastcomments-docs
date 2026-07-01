## Parameters

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| deleteComments | string | query | לא |  |
| commentDeleteMode | string | query | לא |  |

## Response

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## דוגמה

[inline-code-attrs-start title = 'delete_tenant_user דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import DeleteTenantUserOptions
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית והברירה המחדל היא https://fastcomments.com
# ראה configuration.py עבור רשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמשו בדוגמה
# המתאימה למקרה האימות שלכם.

# הגדרת הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטל את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) למפתח API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = 'delete_comments_example' # str |  (optional)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (optional)

    try:
        api_response = api_instance.delete_tenant_user(tenant_id, id, DeleteTenantUserOptions(delete_comments=delete_comments, comment_delete_mode=comment_delete_mode))
        print("התגובה של DefaultApi->delete_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("שגיאה בעת קריאה ל-DefaultApi->delete_tenant_user: %s\n" % e)
[inline-code-end]