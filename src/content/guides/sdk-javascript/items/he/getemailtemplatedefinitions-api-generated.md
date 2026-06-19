---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |

## תגובה

מחזיר: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// פרמטרים אופציונליים (אם נתמכים) יכולים להיות מועברים כארגומנט שני, לדוגמה getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]

---