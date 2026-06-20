## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## תגובה

מחזיר: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ובברירת המחדל https://fastcomments.com
# ראו את configuration.py לרשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# פתחו קונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (אופציונלי)
    direction = client.SortDirections() # SortDirections |  (אופציונלי)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (אופציונלי)
    page = 3.4 # float |  (אופציונלי)
    includei10n = True # bool |  (אופציונלי)
    locale = 'locale_example' # str |  (אופציונלי)
    is_crawler = True # bool |  (אופציונלי)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]