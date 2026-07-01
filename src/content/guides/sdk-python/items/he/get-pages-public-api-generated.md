List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | סמן עמודה עכורה להפלוגה שהוחזר כ‑`nextCursor` מבקשת קודמת. קשור לאותו `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | מסנן קידומת כותרת לא רגיש לאותיות (אופציונלי). |
| sortBy | string | query | No | סדר מיון. `updatedAt` (ברירת מחדל, החדש ביותר ראשון), `commentCount` (התגובות הרבות ביותר ראשון), או `title` (אלפביתי). |
| hasComments | boolean | query | No | אם true, מחזיר רק דפים עם לפחות תגובה אחת. |

## Response

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# הגדרת ה‑host היא אופציונלית ובברירת מחדל https://fastcomments.com
# ראה configuration.py לקבלת רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנס להקשר עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה‑API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | סמן עמודה עכורה להפלוגה שהוחזר כ‑`nextCursor` מבקשת קודמת. קשור לאותו `sortBy`. (אופציונלי)
    limit = 56 # int | 1..200, default 50 (אופציונלי)
    q = 'q_example' # str | מסנן קידומת כותרת לא רגיש לאותיות (אופציונלי)
    sort_by = client.PagesSortBy() # PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, החדש ביותר ראשון), `commentCount` (התגובות הרבות ביותר ראשון), או `title` (אלפביתי). (אופציונלי)
    has_comments = True # bool | אם true, מחזיר רק דפים עם לפחות תגובה אחת. (אופציונלי)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]