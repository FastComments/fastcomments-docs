## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| editKey | string | query | לא |  |

## תגובה

מחזיר: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של delete_vote'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ומוגדרת כברירת מחדל ל- https://fastcomments.com
# ראו את configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמשו בדוגמה שמ
# מתאימה למקרה השימוש שלכם באימות.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטל את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) עבור מפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו להקשר עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    edit_key = 'edit_key_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.delete_vote(tenant_id, id, edit_key=edit_key)
        print("The response of DefaultApi->delete_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_vote: %s\n" % e)
[inline-code-end]