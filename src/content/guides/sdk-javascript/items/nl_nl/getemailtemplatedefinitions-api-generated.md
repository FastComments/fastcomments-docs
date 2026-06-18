---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |

## Antwoord

Retourneert: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_001';
  const options: { includeDrafts?: boolean } = { includeDrafts: true }; // optionele parameter ter demonstratie
  const templates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId, options);
  console.log(templates);
})();
[inline-code-end]

---