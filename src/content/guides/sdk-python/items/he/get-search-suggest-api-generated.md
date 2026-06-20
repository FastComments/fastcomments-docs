## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| text-search | string | שאילתה | לא |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_suggest_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_search_suggest'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_suggest_response import ModerationSuggestResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית ומוגדרת כבררת מחדל ל-https://fastcomments.com
# עיינו ב-configuration.py לרשימה של כל פרמטרי הקונפיגורציה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסו להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_search_suggest(text_search=text_search, sso=sso)
        print("The response of ModerationApi->get_search_suggest:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_suggest: %s\n" % e)
[inline-code-end]