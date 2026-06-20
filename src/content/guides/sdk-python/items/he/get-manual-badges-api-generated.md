## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | לא |  |

## Response

מחזיר: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_manual_badges_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_manual_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_manual_badges_response import GetTenantManualBadgesResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית וברירת המחדל היא https://fastcomments.com
# ראו את configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# פתחו הקשר עם מופע של ApiClient
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_manual_badges(sso=sso)
        print("The response of ModerationApi->get_manual_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_manual_badges: %s\n" % e)
[inline-code-end]