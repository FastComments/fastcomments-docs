## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## תגובה

מחזיר: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_tenant'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_response import GetTenantResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# ראה configuration.py עבור רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# דוגמאות לכל שיטת אימות מסופקות למטה; השתמש בדוגמה שמספקת
# את מערך האימות המתאים למקרה השימוש שלך.

# Configure API key authorization: api_key
# הגדר הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# הכנס הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant(tenant_id, id)
        print("The response of DefaultApi->get_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant: %s\n" % e)
[inline-code-end]