## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| locale | string | query | No |  |

## תגובה

מחזיר: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/render_email_template_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-render_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.render_email_template_body import RenderEmailTemplateBody
from client.models.render_email_template_response import RenderEmailTemplateResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית ובררת המחדל היא https://fastcomments.com
# ראה configuration.py עבור רשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות והרשאות
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמשו בדוגמה שהיא
# מתאימה למקרה השימוש שלכם.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסירו את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) עבור מפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנסו לקונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    render_email_template_body = client.RenderEmailTemplateBody() # RenderEmailTemplateBody | 
    locale = 'locale_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.render_email_template(tenant_id, render_email_template_body, locale=locale)
        print("The response of DefaultApi->render_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->render_email_template: %s\n" % e)
[inline-code-end]