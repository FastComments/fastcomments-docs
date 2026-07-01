## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | שאילתה | כן |  |
| questionId | string | שאילתה | לא |  |
| questionIds | array | שאילתה | לא |  |
| urlId | string | שאילתה | לא |  |
| startDate | string | שאילתה | לא |  |
| forceRecalculate | boolean | שאילתה | לא |  |
| minValue | number | שאילתה | לא |  |
| maxValue | number | שאילתה | לא |  |
| limit | number | שאילתה | לא |  |

## תגובה

מחזיר: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## דוגמה

[inline-code-attrs-start title = 'combine_comments_with_question_results דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import CombineCommentsWithQuestionResultsOptions
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית ובברירת מחדל https://fastcomments.com
# ראה configuration.py עבור רשימת כל הפרמטרים הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# על הלקוח להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של השרת API.
# דוגמאות לכל שיטת אימות ניתנות למטה, השתמשו בדוגמה שמספקת
# עומדת במקרuse של האימות שלכם.

# הגדרת הרשאת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטלו את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) למפתח API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו לקונטקסט עם מופע של לקוח ה-API
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
        api_response = api_instance.combine_comments_with_question_results(tenant_id, CombineCommentsWithQuestionResultsOptions(question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit))
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]