## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| limit | number | query | לא |  |
| skip | number | query | לא |  |
| order | string | query | לא |  |
| after | number | query | לא |  |
| before | number | query | לא |  |

## תגובה

מחזיר: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_audit_logs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs200_response import GetAuditLogs200Response
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית ומשתמשת כברירת מחדל ב-https://fastcomments.com
# ראה את configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מוצגות להלן, השתמש בדוגמה
# שמותאמת למקרה השימוש שלך.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# פתח הקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (אופציונלי)
    skip = 3.4 # float |  (אופציונלי)
    order = client.SORTDIR() # SORTDIR |  (אופציונלי)
    after = 3.4 # float |  (אופציונלי)
    before = 3.4 # float |  (אופציונלי)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]