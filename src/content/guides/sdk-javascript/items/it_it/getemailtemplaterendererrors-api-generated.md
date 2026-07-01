## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetEmailTemplateRenderErrorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateRenderErrorsResponse1.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getEmailTemplateRenderErrors'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c2f5a8d9-4b3e-4f6a-9e1b-2d5c6f7a8b9c";
const templateId: string = "welcome-email-template";
const skip: number = 20;

const result: GetEmailTemplateRenderErrorsResponse1 = await getEmailTemplateRenderErrors(
  tenantId,
  templateId,
  skip
);
[inline-code-end]

---