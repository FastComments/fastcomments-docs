## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getEmailTemplates'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEmailTemplates() {
  const tenantId: string = "acme-corp-123";
  const skip: number = 20;
  const templatesWithSkip: GetEmailTemplatesResponse = await getEmailTemplates(tenantId, skip);
  const templatesWithoutSkip: GetEmailTemplatesResponse = await getEmailTemplates(tenantId);
}
[inline-code-end]

---