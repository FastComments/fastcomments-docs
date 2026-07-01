## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | לא |  |
| userId | string | query | לא |  |
| startDate | string | query | לא |  |
| questionId | string | query | לא |  |
| questionIds | string | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetQuestionResultsOptions
from client.models.get_question_results_response import GetQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ובברירת מחדל https://fastcomments.com
# ראו configuration.py עבור רשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות ניתנות למטה, השתמשו בדוגמה המתאימה
# שתואמת למקרה השימוש שלכם.

# הגדר הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטלו את ההשמטה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# היכנסו להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (אופציונלי)
    user_id = 'user_id_example' # str |  (אופציונלי)
    start_date = 'start_date_example' # str |  (אופציונלי)
    question_id = 'question_id_example' # str |  (אופציונלי)
    question_ids = 'question_ids_example' # str |  (אופציונלי)
    skip = 3.4 # float |  (אופציונלי)

    try:
        api_response = api_instance.get_question_results(tenant_id, GetQuestionResultsOptions(url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip))
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]