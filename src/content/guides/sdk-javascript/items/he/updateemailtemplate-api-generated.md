## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Yes |  |

## תגובה

מחזירה: [`UpdateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateEmailTemplateResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת updateEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "email_tpl_67890";

  const updateBody: UpdateEmailTemplateBody = {
    subject: "Comment reply notification",
    htmlContent: "<p>Someone replied to your comment.</p>",
    plainTextContent: "Someone replied to your comment.",
    // דוגמה לשדה אופציונלי
    isActive: true,
  };

  const result: UpdateEmailTemplateResponse = await updateEmailTemplate(
    tenantId,
    templateId,
    updateBody
  );

  console.log(result);
})();
[inline-code-end]