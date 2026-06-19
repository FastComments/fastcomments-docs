## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |

## Antwoord

Retourneert: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// Optionele parameters (indien ondersteund) kunnen als tweede argument worden meegegeven, bijvoorbeeld getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]