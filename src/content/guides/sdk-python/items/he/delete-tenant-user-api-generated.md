## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| deleteComments | string | query | לא |  |
| commentDeleteMode | string | query | לא |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-delete_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# ראה configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# דוגמאות לכל שיטת אימות מסופקות להלן, השתמש בדוגמה המתאימה למקרה השימוש שלך.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# כניסה להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = 'delete_comments_example' # str |  (optional)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (optional)

    try:
        api_response = api_instance.delete_tenant_user(tenant_id, id, delete_comments=delete_comments, comment_delete_mode=comment_delete_mode)
        print("The response of DefaultApi->delete_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_tenant_user: %s\n" % e)
[inline-code-end]

---