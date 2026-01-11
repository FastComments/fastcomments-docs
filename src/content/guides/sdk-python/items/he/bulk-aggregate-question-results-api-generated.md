## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| forceRecalculate | boolean | query | No |  |

## תגובה

מחזיר: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_aggregate_question_results200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-bulk_aggregate_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_aggregate_question_results200_response import BulkAggregateQuestionResults200Response
from client.models.bulk_aggregate_question_results_request import BulkAggregateQuestionResultsRequest
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית וברירת המחדל היא https://fastcomments.com
# ראה את configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# הדגמות לכל שיטת אימות יינתנו למטה — השתמש בדוגמה שמתאימה למקרה השימוש שלך.

# הגדר אימות באמצעות מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    bulk_aggregate_question_results_request = client.BulkAggregateQuestionResultsRequest() # BulkAggregateQuestionResultsRequest | 
    force_recalculate = True # bool |  (אופציונלי)

    try:
        api_response = api_instance.bulk_aggregate_question_results(tenant_id, bulk_aggregate_question_results_request, force_recalculate=force_recalculate)
        print("The response of DefaultApi->bulk_aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->bulk_aggregate_question_results: %s\n" % e)
[inline-code-end]

---