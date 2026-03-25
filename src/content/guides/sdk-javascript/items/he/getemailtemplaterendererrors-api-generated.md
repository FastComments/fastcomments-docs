---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| skip | number | לא |  |

## תגובה

מחזיר: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateRenderErrors200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getEmailTemplateRenderErrors'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-tenant-42';
  const id: string = 'tmpl_3fa85f64-5717-4562-b3fc-2c963f66afa6';
  const skip: number = 20;
  const result: GetEmailTemplateRenderErrors200Response = await getEmailTemplateRenderErrors(tenantId, id, skip);
  console.log(result);
})();
[inline-code-end]

---