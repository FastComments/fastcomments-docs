## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| commentId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_internal_profile_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_internal_profile'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_internal_profile_response import GetUserInternalProfileResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# ראו configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_user_internal_profile(comment_id=comment_id, sso=sso)
        print("The response of ModerationApi->get_user_internal_profile:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_user_internal_profile: %s\n" % e)
[inline-code-end]

---