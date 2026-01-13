## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderator200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_moderator'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderator200_response import GetModerator200Response
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית והברירת מחדל היא https://fastcomments.com
# ראה את configuration.py לרשימה של כל פרמטרי התצורה הנתמכים.
# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות להלן, השתמש בדוגמה ש
# מספקת את צרכי האימות שלך.
# Configure API key authorization: api_key
# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_moderator(tenant_id, id)
        print("The response of DefaultApi->get_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderator: %s\n" % e)
[inline-code-end]