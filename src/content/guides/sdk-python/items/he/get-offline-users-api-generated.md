---
המתגיבים הקודמים בעמוד שאינם מחוברים כרגע. ממוינים לפי displayName.  
השתמשו בזה לאחר שניצל /users/online כדי להציג מדור "Members".  
דפדוף באמצעות מצביע על commenterName: השרת הולך בחלק החלקי {tenantId, urlId, commenterName}  
ממיקום afterName קדימה באמצעות $gt, ללא עלות של $skip.

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה URL של העמוד (נוקה בצד השרת). |
| afterName | string | query | No | מצביע: העבירו nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | No | מפצל מצביע: העבירו nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של קישוריות שמות לא יגרמו להסרת רשומות. |

## תשובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## דוגמה

[inline-code-attrs-start title = 'get_offline_users דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית ובברירת מחדל היא https://fastcomments.com
# ראו configuration.py עבור רשימת כל פרמטרי הקונפיגורציה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# נכנסו לקונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | מזהה URL של העמוד (נוקה בצד השרת).
    after_name = 'after_name_example' # str | מצביע: העבירו nextAfterName מהתגובה הקודמת. (אופציונלי)
    after_user_id = 'after_user_id_example' # str | מפצל מצביע: העבירו nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של קישוריות שמות לא יגרמו להסרת רשומות. (אופציונלי)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]