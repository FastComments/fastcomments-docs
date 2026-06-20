---
רשימת דפים עבור שוכר. משמש על ידי לקוח שולחני של FChat כדי למלא את רשימת החדרים שלו.
דורש ש־`enableFChat` יהיה true בקונפיגורציית ה-custom שנפתרה עבור כל דף.
דפים שדורשים SSO מסוננים לפי גישת הקבוצות של המשתמש המבקש.

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| cursor | string | query | לא | סמן דפדוף אטום שהוחזר כ־`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. |
| limit | integer | query | לא | 1..200, ברירת מחדל 50 |
| q | string | query | לא | מסנן קידומת לכותרת שאינו רגיש לרישיות. |
| sortBy | string | query | לא | סדר מיון. `updatedAt` (ברירת מחדל, החדשים ביותר תחילה), `commentCount` (ראשית הדפים עם הכי הרבה תגובות), או `title` (בסדר אלפביתי). |
| hasComments | boolean | query | לא | אם true, החזר רק דפים עם לפחות תגובה אחת. |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראו configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# כניסה לקונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | סמן דפדוף אטום שהוחזר כ־`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. (אופציונלי)
    limit = 56 # int | 1..200, ברירת מחדל 50 (אופציונלי)
    q = 'q_example' # str | מסנן קידומת לכותרת שאינו רגיש לרישיות. (אופציונלי)
    sort_by = client.PagesSortBy() # PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, החדשים ביותר תחילה), `commentCount` (ראשית הדפים עם הכי הרבה תגובות), או `title` (בסדר אלפביתי). (אופציונלי)
    has_comments = True # bool | אם true, החזר רק דפים עם לפחות תגובה אחת. (אופציונלי)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---