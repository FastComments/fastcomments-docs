---
רשימת דפים עבור שוכר. משמש את לקוח השולחן העבודה FChat למלא את רשימת החדרים שלו.
דורש `enableFChat` להיות true בתצורה המותאמת המותאמת לכל דף.
דפים הדורשים SSO מסוננים בהתאם לגישת קבוצת המשתמש המבקש.

## פרמטרים

| שם | סוג | נדרש | תיאור |
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
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]

---