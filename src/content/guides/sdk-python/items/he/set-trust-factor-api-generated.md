## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | לא |  |
| trustFactor | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_user_trust_factor_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-set_trust_factor'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_user_trust_factor_response import SetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית וערך ברירת המחדל הוא https://fastcomments.com
# ראה את configuration.py לרשימת כל הפרמטרים הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# כניסה להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (אופציונלי)
    trust_factor = 'trust_factor_example' # str |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.set_trust_factor(user_id=user_id, trust_factor=trust_factor, sso=sso)
        print("The response of ModerationApi->set_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->set_trust_factor: %s\n" % e)
[inline-code-end]

---