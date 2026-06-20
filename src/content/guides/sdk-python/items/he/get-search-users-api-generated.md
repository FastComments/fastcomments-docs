## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | No |  |
| sso | string | query | No |  |

## תגובה

מחזיר: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_user_search_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_search_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_user_search_response import ModerationUserSearchResponse
from client.rest import ApiException
from pprint import pprint

# קביעת ה-host היא אופציונלית והברירת מחדל היא https://fastcomments.com
# עיין ב-configuration.py לקבלת רשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# פתיחת הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_search_users(value=value, sso=sso)
        print("The response of ModerationApi->get_search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_users: %s\n" % e)
[inline-code-end]