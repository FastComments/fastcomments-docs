רשימת דפים עבור טננט. מנוצל על ידי לקוח שולחני של FChat כדי למלא את רשימת החדרים שלו.
דורש ש־`enableFChat` יהיה true על הקונפיגורציה המותאמת שהתקבלה עבור כל דף.
דפים שדורשים SSO מסוננים לפי גישת הקבוצות של המשתמש המבקש.

## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| cursor | string | לא |  |
| limit | int | לא |  |
| q | string | לא |  |
| sortBy | PagesSortBy | לא |  |
| hasComments | bool | לא |  |

## תגובה

מחזיר: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(
  tenantId = "my-tenant-123",
  cursor = "",
  limit = 0,
  q = "",
  sortBy = PagesSortBy(0),
  hasComments = false
)

if response.isSome:
  let pages = response.get()
  echo "Retrieved public pages: ", $pages
else:
  echo "No pages returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---