## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| updateComments | string | query | לא |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.update_tenant_user_body import UpdateTenantUserBody
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית והערך המחדל הוא https://fastcomments.com
# ראה configuration.py לרשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח צריך להגדיר את פרמטרי האימות והרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה שמ
# מתאימה למקרה השימוש שלך.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# היכנס לקונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_tenant_user_body = client.UpdateTenantUserBody() # UpdateTenantUserBody | 
    update_comments = 'update_comments_example' # str |  (optional)

    try:
        api_response = api_instance.update_tenant_user(tenant_id, id, update_tenant_user_body, update_comments=update_comments)
        print("The response of DefaultApi->update_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_tenant_user: %s\n" % e)
[inline-code-end]