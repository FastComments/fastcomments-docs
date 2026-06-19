## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| skip | number | לא |  |

## תגובה

מחזיר: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsersResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenantUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_7f3b2a9c';
  const skip: number = 20; // פרמטר אופציונלי להדגמה
  const result: GetTenantUsersResponse = await getTenantUsers(tenantId, skip);
  console.log(result);
})();
[inline-code-end]

---