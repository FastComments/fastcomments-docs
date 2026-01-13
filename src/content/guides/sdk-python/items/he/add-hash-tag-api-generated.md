## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | לא |  |

## תגובה

מחזיר: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tag200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-add_hash_tag'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tag200_response import AddHashTag200Response
from client.models.create_hash_tag_body import CreateHashTagBody
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית וברירת המחדל היא https://fastcomments.com
# עיין ב-configuration.py כדי לראות רשימה של כל פרמטרי התצורה הנתמכים.
# על הלקוח להגדיר את פרמטרי האימות והרשאות
# בהתאם למדיניות אבטחת שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה — השתמש בדוגמה ש
# מתאימה למקרה השימוש שלך.
# הגדר אימות באמצעות מפתח API: api_key
# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# פתח בלוק הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (אופציונלי)
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (אופציונלי)

    try:
        api_response = api_instance.add_hash_tag(tenant_id=tenant_id, create_hash_tag_body=create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]