---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| search | string | כן |  |
| locale | string | לא |  |
| rating | string | לא |  |
| page | number | לא |  |

## תגובה

מחזיר: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_fcm_42";
const search: string = "funny golden retriever";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 2;
const result: GifSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---