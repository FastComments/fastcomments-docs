מפרט עמודים עבור טננט. משמש את לקוח שולחן העבודה FChat לאכלוס רשימת החדרים שלו.
דורש ש-`enableFChat` יהיה true בתצורה המותאמת שנפתרה עבור כל עמוד.
עמודים הדורשים SSO מסוננים מול הרשאות הקבוצה של המשתמש המבקש.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| cursor | string | query | לא | סמן עימוד אטום שהוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. |
| limit | integer | query | לא | 1..200, ברירת מחדל 50 |
| q | string | query | לא | מסנן תחילית כותרת אופציונלי, ללא תלות ברישיות. |
| sortBy | string | query | לא | סדר מיון. `updatedAt` (ברירת מחדל, החדש ביותר ראשון), `commentCount` (הכי הרבה תגובות ראשון), או `title` (אלפביתי). |
| hasComments | boolean | query | לא | אם true, מחזיר רק עמודים עם לפחות תגובה אחת. |

## Response

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'דוגמה ל-get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | סמן עימוד אטום שהוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. (optional)
    limit = 56 # int | 1..200, ברירת מחדל 50 (optional)
    q = 'q_example' # str | מסנן תחילית כותרת אופציונלי, ללא תלות ברישיות. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, החדש ביותר ראשון), `commentCount` (הכי הרבה תגובות ראשון), או `title` (אלפביתי). (optional)
    has_comments = True # bool | אם true, מחזיר רק עמודים עם לפחות תגובה אחת. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]