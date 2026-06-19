## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises-1";
const templateId: string = "welcome-onboard-v2";
const result: GetEmailTemplateResponse = await getEmailTemplate(tenantId, templateId);
const status: APIStatus | undefined = result.status;
const template: CustomEmailTemplate | undefined = result.template;
const subject: string | undefined = template?.subject
[inline-code-end]

---