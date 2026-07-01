## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie : [`GetEmailTemplateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTemplate(): Promise<void> {
    const tenantId: string = "tenant-12345";
    const templateId: string = "order-confirmation";
    const response: GetEmailTemplateResponse1 = await getEmailTemplate(tenantId, templateId);
    const emailTemplate: CustomEmailTemplate | undefined = response.customEmailTemplate;
    const configParams: CustomConfigParameters | undefined = response.customConfigParameters;
}
[inline-code-end]