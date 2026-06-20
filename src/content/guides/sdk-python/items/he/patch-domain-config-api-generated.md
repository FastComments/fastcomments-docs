## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| domainToUpdate | string | path | כן |  |

## תגובה

מחזיר: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_domain_config_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-patch_domain_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_domain_config_params import PatchDomainConfigParams
from client.models.patch_domain_config_response import PatchDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראו את configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות ניתנות למטה, השתמשו בדוגמה ש-
# מתאימה למקרה השימוש שלכם.

# הגדרות האישור בעזרת מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסירו את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם צריך
# configuration.api_key_prefix['api_key'] = 'Bearer'

# צרו קונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain_to_update = 'domain_to_update_example' # str | 
    patch_domain_config_params = client.PatchDomainConfigParams() # PatchDomainConfigParams | 

    try:
        api_response = api_instance.patch_domain_config(tenant_id, domain_to_update, patch_domain_config_params)
        print("The response of DefaultApi->patch_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_domain_config: %s\n" % e)
[inline-code-end]