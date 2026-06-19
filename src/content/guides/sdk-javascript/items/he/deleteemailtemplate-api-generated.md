## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const templateId: string = 'tmpl_3fa85f64-5717-4562-b3fc-2c963f66afa6';
const optionalStatus: APIStatus | undefined = undefined;
const result: APIEmptyResponse = await deleteEmailTemplate(tenantId, templateId);
[inline-code-end]