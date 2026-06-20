## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| page | integer | query | לא |  |
| limit | integer | query | לא |  |
| skip | integer | query | לא |  |
| asTree | boolean | query | לא |  |
| skipChildren | integer | query | לא |  |
| limitChildren | integer | query | לא |  |
| maxTreeDepth | integer | query | לא |  |
| urlId | string | query | לא |  |
| userId | string | query | לא |  |
| anonUserId | string | query | לא |  |
| contextUserId | string | query | לא |  |
| hashTag | string | query | לא |  |
| parentId | string | query | לא |  |
| direction | string | query | לא |  |
| fromDate | integer | query | לא |  |
| toDate | integer | query | לא |  |

## תגובה

מחזיר: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית והיא כברירת מחדל https://fastcomments.com
# ראה את configuration.py עבור רשימת כל הפרמטרים הנתמכים של התצורה.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בעוגמה אשר
# מתאימה למקרה השימוש שלך.

# הגדר הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם יש צורך
# configuration.api_key_prefix['api_key'] = 'Bearer'

# היכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (אופציונלי)
    limit = 56 # int |  (אופציונלי)
    skip = 56 # int |  (אופציונלי)
    as_tree = True # bool |  (אופציונלי)
    skip_children = 56 # int |  (אופציונלי)
    limit_children = 56 # int |  (אופציונלי)
    max_tree_depth = 56 # int |  (אופציונלי)
    url_id = 'url_id_example' # str |  (אופציונלי)
    user_id = 'user_id_example' # str |  (אופציונלי)
    anon_user_id = 'anon_user_id_example' # str |  (אופציונלי)
    context_user_id = 'context_user_id_example' # str |  (אופציונלי)
    hash_tag = 'hash_tag_example' # str |  (אופציונלי)
    parent_id = 'parent_id_example' # str |  (אופציונלי)
    direction = client.SortDirections() # SortDirections |  (אופציונלי)
    from_date = 56 # int |  (אופציונלי)
    to_date = 56 # int |  (אופציונלי)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]