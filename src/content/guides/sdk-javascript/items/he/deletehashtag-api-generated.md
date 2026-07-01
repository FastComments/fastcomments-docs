## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tag | string | כן |  |
| tenantId | string | לא |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | לא |  |

## תגובה

מחזיר: [`DeleteHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteHashTagResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת deleteHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tag: string = "announcement";
  const tenantId: string = "tenant_9876";
  const requestBody: DeleteHashTagRequestBody = {
    confirmDeletion: true
  };
  const response: DeleteHashTagResponse = await deleteHashTag(tag, tenantId, requestBody);
  console.log(response);
})();
[inline-code-end]

---