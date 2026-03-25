## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-marketing-042";
const templateId: string = "tpl_welcome_2026";
const result: GetEmailTemplate200Response = await getEmailTemplate(tenantId, templateId);
const template: CustomEmailTemplate | undefined = result.template;
const subject: string | undefined = template?.subject;
const customParams: CustomConfigParameters | undefined = template?.customConfigParameters;
[inline-code-end]