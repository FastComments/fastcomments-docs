## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| postIds | array | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/user_reacts_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_reacts_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.user_reacts_response import UserReactsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית והברירת מחדל היא https://fastcomments.com
# עיין ב-configuration.py לרשימה של כל הפרמטרים הנתמכים להגדרה.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# כניסה לקונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_ids = ['post_ids_example'] # List[str] |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_user_reacts_public(tenant_id, post_ids=post_ids, sso=sso)
        print("The response of PublicApi->get_user_reacts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_reacts_public: %s\n" % e)
[inline-code-end]

---