מאחד מסמכים על ידי קיבוץ שלהם (אם groupBy מסופק) והפעלת מספר פעולות.
נתמכות פעולות שונות (למשל sum, countDistinct, avg וכו').

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | שאילתה | כן |  |
| parentTenantId | string | שאילתה | לא |  |
| includeStats | boolean | שאילתה | לא |  |

## תגובה

מחזיר: [`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-aggregate'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית ובברירת המחדל היא https://fastcomments.com
# ראה את configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# הדוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה המתאימה
# למקרה השימוש שלך.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (אופציונלי)
    include_stats = True # bool |  (אופציונלי)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]