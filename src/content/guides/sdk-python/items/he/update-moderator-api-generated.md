## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_moderator'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.update_moderator_body import UpdateModeratorBody
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ובברירת המחדל היא https://fastcomments.com
# ראה configuration.py לרשימת כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על ה-client להגדיר את פרמטרי האימות והרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# להלן מסופקות דוגמאות לכל שיטת אימות; השתמשו בדוגמה המתאימה
# למקרה השימוש שלכם.
# הגדר הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, במידה ונדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס לקונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_moderator_body = client.UpdateModeratorBody() # UpdateModeratorBody | 

    try:
        api_response = api_instance.update_moderator(tenant_id, id, update_moderator_body)
        print("The response of DefaultApi->update_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_moderator: %s\n" % e)
[inline-code-end]