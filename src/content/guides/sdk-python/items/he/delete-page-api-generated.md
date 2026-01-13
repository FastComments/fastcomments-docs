## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_page_api_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-delete_page'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_page_api_response import DeletePageAPIResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אינה חובה וברירת המחדל היא https://fastcomments.com
# ראו configuration.py עבור רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות עבור כל שיטת אימות מסופקות להלן, השתמשו בדוגמה שמספקת
# את דרישות השימוש שלכם באימות.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_page(tenant_id, id)
        print("The response of DefaultApi->delete_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_page: %s\n" % e)
[inline-code-end]