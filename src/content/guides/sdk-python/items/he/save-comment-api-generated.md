## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## תגובה

מחזיר: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_save_comment_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל‑save_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import SaveCommentOptions
from client.models.api_save_comment_response import APISaveCommentResponse
from client.models.create_comment_params import CreateCommentParams
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ובברירת המחדל היא https://fastcomments.com
# ראה את הקובץ configuration.py עבור רשימה של כל פרמטרי תצורה נתמכים.

# על הלקוח להגדיר את פרמטרי האימות והאישור בהתאם למדיניות האבטחה של שרת ה‑API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמשו בדוגמה שמתאימה למקרה השימוש שלכם.

# הגדרת הרשאת מפתח API: api_key
# בטלו את ההערה מהשורה למטה כדי להגדיר מקדמה (למשל Bearer) עבור מפתח API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס לקונטקסט עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה‑API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = client.CreateCommentParams() # CreateCommentParams | 
    is_live = True # bool |  (אופציונלי)
    do_spam_check = True # bool |  (אופציונלי)
    send_emails = True # bool |  (אופציונלי)
    populate_notifications = True # bool |  (אופציונלי)

    try:
        api_response = api_instance.save_comment(tenant_id, create_comment_params, SaveCommentOptions(is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications))
        print("The response of DefaultApi->save_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comment: %s\n" % e)
[inline-code-end]