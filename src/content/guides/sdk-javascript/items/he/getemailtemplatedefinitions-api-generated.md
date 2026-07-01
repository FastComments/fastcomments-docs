## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## תגובה

מחזיר: [`GetEmailTemplateDefinitionsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse1.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const emailTemplateDefs: GetEmailTemplateDefinitionsResponse1 = await getEmailTemplateDefinitions(tenantId);
  console.log(emailTemplateDefs);
})();
[inline-code-end]