## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## תגובה

מחזיר: [`GetUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserResponse1.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_98765";
  const result: GetUserResponse1 = await getUser(tenantId, userId);
})();
[inline-code-end]

---