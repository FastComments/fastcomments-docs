## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## תגובה

מחזיר: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/put_domain_config_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת put_domain_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.put_domain_config_response import PutDomainConfigResponse
from client.models.update_domain_config_params import UpdateDomainConfigParams
from client.rest import ApiException
from pprint import pprint

# הגדרת host היא אופציונלית וברירת המחדל היא https://fastcomments.com
# ראו את configuration.py לרשימת כל פרמטרי הקונפיגורציה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# ה-client חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מוצגות למטה, השתמשו בדוגמה ש
# מתאימה למקרה השימוש שלכם.

# הגדרת הרשאת מפתח ה-API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסירו את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# כניסה להקשר עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain_to_update = 'domain_to_update_example' # str | 
    update_domain_config_params = client.UpdateDomainConfigParams() # UpdateDomainConfigParams | 

    try:
        api_response = api_instance.put_domain_config(tenant_id, domain_to_update, update_domain_config_params)
        print("The response of DefaultApi->put_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->put_domain_config: %s\n" % e)
[inline-code-end]