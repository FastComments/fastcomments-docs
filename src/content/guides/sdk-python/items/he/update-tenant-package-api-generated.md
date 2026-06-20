## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_tenant_package'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_tenant_package_body import UpdateTenantPackageBody
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ומוגדרת כברירת מחדל ל־https://fastcomments.com
# עיין בקובץ configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטרי האימות והרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# למטה יש דוגמאות לכל שיטת אימות; השתמש בדוגמה שמ
# עונה על מקרה השימוש שלך.
# הגדר אימות באמצעות מפתח API: api_key
# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם צריך
# configuration.api_key_prefix['api_key'] = 'Bearer'

# פתח הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_tenant_package_body = client.UpdateTenantPackageBody() # UpdateTenantPackageBody | 

    try:
        api_response = api_instance.update_tenant_package(tenant_id, id, update_tenant_package_body)
        print("The response of DefaultApi->update_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_tenant_package: %s\n" % e)
[inline-code-end]

---