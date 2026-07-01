## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_create_hash_tags_response.py)

## דוגמה

[inline-code-attrs-start title = 'add_hash_tags_bulk דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.models.bulk_create_hash_tags_response import BulkCreateHashTagsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית וכברירת מחדל https://fastcomments.com
# ראה configuration.py עבור רשימת כל פרמטרי התצורה הנתמכים.
# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמשו בדוגמה שעונה על
# הצרכים שלכם.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# בטלו את ההערה שלהלן כדי להגדיר קידומת (לדוגמה Bearer) למפתח ה-API, אם נדרש
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (אופציונלי)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id, bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]