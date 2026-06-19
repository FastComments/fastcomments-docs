## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Odgovor

Vraća: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// Neobavezni parametri (ako su podržani) mogu se proslijediti kao drugi argument, npr. getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]