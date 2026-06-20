## פרמטרים

| שם | Type | Location | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | query | לא |  |
| externalId | string | query | לא |  |
| eventType | string | query | לא |  |
| type | string | query | לא |  |
| domain | string | query | לא |  |
| attemptCountGT | number | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_pending_webhook_events'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_events_response import GetPendingWebhookEventsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית וברירת המחדל היא https://fastcomments.com
# ראה configuration.py לרשימה של כל פרמטרי ההגדרה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות והרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה ש
# מספקת את המקרה השימושי שלך.

# הגדר הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה להלן על מנת להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (אופציונלי)
    external_id = 'external_id_example' # str |  (אופציונלי)
    event_type = 'event_type_example' # str |  (אופציונלי)
    type = 'type_example' # str |  (אופציונלי)
    domain = 'domain_example' # str |  (אופציונלי)
    attempt_count_gt = 3.4 # float |  (אופציונלי)
    skip = 3.4 # float |  (אופציונלי)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip)
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]