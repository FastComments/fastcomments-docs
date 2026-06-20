---
## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| isLive | boolean | query | לא |  |
| doSpamCheck | boolean | query | לא |  |
| sendEmails | boolean | query | לא |  |
| populateNotifications | boolean | query | לא |  |

## תגובה

מחזיר: [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comments_bulk_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-save_comments_bulk'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comments_bulk_response import SaveCommentsBulkResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית והבררת המחדל היא https://fastcomments.com
# ראה configuration.py לרשימת כל הפרמטרים הנתמכים.
# הלקוח חייב להגדיר את פרמטרי האימות והרשאה
# בהתאם למדיניות אבטחת השרת של ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה ש
# עונה על מקרה השימוש שלך באימות.

# Configure API key authorization: api_key
# הסר הערה להלן כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = [client.CreateCommentParams()] # List[CreateCommentParams] | 
    is_live = True # bool |  (אופציונלי)
    do_spam_check = True # bool |  (אופציונלי)
    send_emails = True # bool |  (אופציונלי)
    populate_notifications = True # bool |  (אופציונלי)

    try:
        api_response = api_instance.save_comments_bulk(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comments_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comments_bulk: %s\n" % e)
[inline-code-end]

---