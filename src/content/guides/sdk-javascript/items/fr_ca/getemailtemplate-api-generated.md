## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Retourne : [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b2c';
const templateId: string = 'welcome-email-2024';
const includeDrafts: boolean | undefined = undefined;
const emailTemplate: GetEmailTemplate200Response = await getEmailTemplate(tenantId, templateId);
[inline-code-end]

---