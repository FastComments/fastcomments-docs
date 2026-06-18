## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vrača: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## Primer

[inline-code-attrs-start title = 'getEmailTemplate Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b2c';
const templateId: string = 'welcome-email-2024';
const includeDrafts: boolean | undefined = undefined;
const emailTemplate: GetEmailTemplate200Response = await getEmailTemplate(tenantId, templateId);
[inline-code-end]

---