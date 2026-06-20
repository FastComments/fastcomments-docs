## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| afterId | string | query | לא |  |
| demo | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comment_ids_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_api_ids'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comment_ids_response import ModerationAPIGetCommentIdsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית והברירת מחדל היא https://fastcomments.com
# ראו configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# כניסה להקשר עם מופע של ApiClient
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (אופציונלי)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (אופציונלי)
    filters = 'filters_example' # str |  (אופציונלי)
    search_filters = 'search_filters_example' # str |  (אופציונלי)
    after_id = 'after_id_example' # str |  (אופציונלי)
    demo = True # bool |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_api_ids(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, after_id=after_id, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_ids: %s\n" % e)
[inline-code-end]