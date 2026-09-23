## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | כן |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת updateEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_67890";

  const updateBody: UpdateEmailTemplateBody = {
    subject: "New Comment Notification",
    // htmlContent הוא אופציונלי ולא נכלל
    status: { code: 200, message: "Active" } // APIStatus
  };

  const response: APIEmptyResponse = await updateEmailTemplate(tenantId, templateId, updateBody);
  console.log(response);
}
[inline-code-end]