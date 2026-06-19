## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |

## Antwort

Gibt zurück: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// Optionale Parameter (falls unterstützt) können als zweites Argument übergeben werden, z.B. getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]