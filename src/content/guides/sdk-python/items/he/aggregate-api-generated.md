Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations. Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| parentTenantId | string | query | No |  |
| includeStats | boolean | query | No |  |

## Response

מחזירה: [`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## Example

[inline-code-attrs-start title = 'דוגמת אגירה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import AggregateOptions
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית והערך ברירת המחדל הוא https://fastcomments.com
# ראה configuration.py לקבלת רשימה של כל פרמטרי ההגדרות הנתמכים.
# על הלקוח להגדיר את פרמטרי האימות והאישור
# בהתאם למדיניות האבטחה של השרת.
# דוגמאות לכל שיטת אימות מוצגות למטה, השתמשו בדוגמה
# המתאימה למקרה השימוש שלכם.

# הגדרת אימות מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטלו את ההערה למטה כדי להוסיף קידומת (למשל Bearer) למפתח ה‑API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו להקשר עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה‑API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (אופציונלי)
    include_stats = True # bool |  (אופציונלי)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, AggregateOptions(parent_tenant_id=parent_tenant_id, include_stats=include_stats))
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]