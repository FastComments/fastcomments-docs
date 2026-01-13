## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_page_api_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת add_page'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_page_api_response import AddPageAPIResponse
from client.models.create_api_page_data import CreateAPIPageData
from client.rest import ApiException
from pprint import pprint

# קביעת ה-host אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראה configuration.py לרשימת כל הפרמטרים הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות אבטחת השרת של ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה שמתאימה
# לצורך המקרה שלך.

# הגדר אישור מפתח API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# הסר הערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_api_page_data = client.CreateAPIPageData() # CreateAPIPageData | 

    try:
        api_response = api_instance.add_page(tenant_id, create_api_page_data)
        print("The response of DefaultApi->add_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_page: %s\n" % e)
[inline-code-end]

---