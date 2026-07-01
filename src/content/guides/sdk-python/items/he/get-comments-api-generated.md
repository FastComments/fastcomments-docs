## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
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

מחזירה: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetCommentsOptions
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ובברירת מחדל https://fastcomments.com
# ראה configuration.py עבור רשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמשו בדוגמה שת
# תענה על צרכי האימות שלכם.

# הגדרת אימות עם מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטלו את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (optional)
    limit = 56 # int |  (optional)
    skip = 56 # int |  (optional)
    as_tree = True # bool |  (optional)
    skip_children = 56 # int |  (optional)
    limit_children = 56 # int |  (optional)
    max_tree_depth = 56 # int |  (optional)
    url_id = 'url_id_example' # str |  (optional)
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)
    context_user_id = 'context_user_id_example' # str |  (optional)
    hash_tag = 'hash_tag_example' # str |  (optional)
    parent_id = 'parent_id_example' # str |  (optional)
    direction = client.SortDirections() # SortDirections |  (optional)
    from_date = 56 # int |  (optional)
    to_date = 56 # int |  (optional)

    try:
        api_response = api_instance.get_comments(tenant_id, GetCommentsOptions(page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date))
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]