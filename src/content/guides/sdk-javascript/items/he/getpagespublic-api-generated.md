רשימת עמודים עבור tenant. משמש על־ידי לקוח שולחני של FChat כדי למלא את רשימת החדרים שלו.
דורש `enableFChat` להיות `true` בהגדרות המותאמות שנקבעו לכל עמוד.
עמודים שדורשים SSO מסוננים בהתבסס על גישת הקבוצות של המשתמש המבקש.

## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| cursor | string | לא |  |
| limit | number | לא |  |
| q | string | לא |  |
| sortBy | PagesSortBy | לא |  |
| hasComments | boolean | לא |  |

## תגובה

מחזיר: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]