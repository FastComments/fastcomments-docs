## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| questionId | string | query | לא |  |
| questionIds | array | query | לא |  |
| urlId | string | query | לא |  |
| startDate | string | query | לא |  |
| forceRecalculate | boolean | query | לא |  |
| minValue | number | query | לא |  |
| maxValue | number | query | לא |  |
| limit | number | query | לא |  |

## תגובה

מחזיר: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-combine_comments_with_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראו configuration.py לרשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות להלן, השתמשו בדוגמה ש-
# מתאימה למקרה השימוש שלכם.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסירו את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו לקונטקסט עם מופע של ApiClient
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (אופציונלי)
    question_ids = ['question_ids_example'] # List[str] |  (אופציונלי)
    url_id = 'url_id_example' # str |  (אופציונלי)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (אופציונלי)
    force_recalculate = True # bool |  (אופציונלי)
    min_value = 3.4 # float |  (אופציונלי)
    max_value = 3.4 # float |  (אופציונלי)
    limit = 3.4 # float |  (אופציונלי)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]