## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nee |  |

## Antwoord

Retourneert: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getEmailTemplates Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2a9c';
const templatesPage1: GetEmailTemplatesResponse = await getEmailTemplates(tenantId);
const templatesPage2: GetEmailTemplatesResponse = await getEmailTemplates(tenantId, 25);
console.log(templatesPage1, templatesPage2);
[inline-code-end]

---