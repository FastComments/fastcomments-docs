## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| commentId | string | path | כן |  |
| broadcastId | string | query | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`PinComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/pin_comment200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-un_pin_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.pin_comment200_response import PinComment200Response
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראו configuration.py בשביל רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסו להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.un_pin_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->un_pin_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->un_pin_comment: %s\n" % e)
[inline-code-end]