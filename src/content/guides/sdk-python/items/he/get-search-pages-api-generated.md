## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| value | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_page_search_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_search_pages'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_page_search_response import ModerationPageSearchResponse
from client.rest import ApiException
from pprint import pprint

# קביעת host היא אופציונלית וברירת המחדל היא https://fastcomments.com
# עיין ב-configuration.py כדי לראות את כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# כניסה להקשר עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_search_pages(value=value, sso=sso)
        print("The response of ModerationApi->get_search_pages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_pages: %s\n" % e)
[inline-code-end]