## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getEmailTemplates Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEmailTemplates() {
  const tenantId: string = "acme-corp-123";
  const skip: number = 20;
  const templatesWithSkip: GetEmailTemplatesResponse = await getEmailTemplates(tenantId, skip);
  const templatesWithoutSkip: GetEmailTemplatesResponse = await getEmailTemplates(tenantId);
}
[inline-code-end]