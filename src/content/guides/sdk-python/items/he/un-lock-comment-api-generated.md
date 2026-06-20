## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| commentId | string | path | כן |  |
| broadcastId | string | query | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-un_lock_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# קביעת ה-host היא אופציונלית ומוגדרת כברירת מחדל ל- https://fastcomments.com
# ראה את configuration.py עבור רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# פתח בלוק הקשר (context) עם מופע של ApiClient
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.un_lock_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->un_lock_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->un_lock_comment: %s\n" % e)
[inline-code-end]