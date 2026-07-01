## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |

## תגובה

מחזיר: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_pending_webhook_event_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetPendingWebhookEventCountOptions
from client.models.get_pending_webhook_event_count_response import GetPendingWebhookEventCountResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ובברירת מחדל https://fastcomments.com
# ראו configuration.py לרשימת כל פרמטרי תצורה נתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מוצגות למטה, השתמשו בדוגמה המתאימה למקרה השימוש שלכם.

# הגדרת הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסירו את ההערה מתחת כדי להגדיר קידומת (לדוגמה Bearer) למפתח API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו להקשר עם מופע של לקוח ה-API
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

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, GetPendingWebhookEventCountOptions(comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt))
        print("התגובות של DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("חריגה בעת קריאה ל-DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]