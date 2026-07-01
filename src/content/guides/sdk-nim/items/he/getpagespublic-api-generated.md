רשימת דפים עבור שוכר. משמש את לקוח ה‑desktop של FChat למלא את רשימת החדרים שלו. דורש `enableFChat` להיות true בתצורה המותאמת שנפתרה עבור כל דף. דפים הדורשים SSO מסוננים על פי גישת קבוצת המשתמש המבקש.

## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| options | GetPagesPublicOptions | לא |  |

## Response

מחזיר: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]