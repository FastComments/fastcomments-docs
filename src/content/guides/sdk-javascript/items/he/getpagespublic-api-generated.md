רשימת דפים עבור שוכר. משמש על ידי הלקוח השולחני של FChat כדי למלא את רשימת החדרים שלו.
נדרש ש-`enableFChat` יהיה true בתצורה המותאמת שנפתרה עבור כל דף.
דפים שדורשים SSO נסננים בהתאם לגישת הקבוצה של המשתמש המבקש.

## פרמטרים

| שם | סוג | נדרש | תאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| cursor | string | לא |  |
| limit | number | לא |  |
| q | string | לא |  |
| sortBy | PagesSortBy | לא |  |
| hasComments | boolean | לא |  |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---