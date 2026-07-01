רשימת דפים לשוכר. משמשת על ידי לקוח הדסקטופ של FChat למלא את רשימת החדרים שלו.  
דורש `enableFChat` להיות true בתצורה המותאמת שנפתרה עבור כל דף.  
דפים הדורשים SSO מסוננים לפי גישה לקבוצת המשתמש המבקש.

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

מחזירה: [`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'getPagesPublic דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "nextPageToken";
  const limit: number = 20;
  const q: string = "blog";
  const sortBy: PagesSortBy = "createdAt";
  const hasComments: boolean = true;

  const response: GetPagesPublicResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    q,
    sortBy,
    hasComments
  );

  console.log(response);
}
[inline-code-end]