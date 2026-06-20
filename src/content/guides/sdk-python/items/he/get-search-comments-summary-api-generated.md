## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_comment_search_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_search_comments_summary'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_comment_search_response import ModerationCommentSearchResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית והברירת מחדל היא https://fastcomments.com
# ראה את configuration.py עבור רשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (אופציונלי)
    filters = 'filters_example' # str |  (אופציונלי)
    search_filters = 'search_filters_example' # str |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_search_comments_summary(value=value, filters=filters, search_filters=search_filters, sso=sso)
        print("The response of ModerationApi->get_search_comments_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_comments_summary: %s\n" % e)
[inline-code-end]