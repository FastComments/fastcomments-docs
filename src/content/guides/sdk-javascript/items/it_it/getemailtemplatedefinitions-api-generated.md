---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |

## Risposta

Restituisce: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Esempio

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Esempio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_eu_01';
const templates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId);
console.log('Email template definitions loaded for', tenantId, templates);
[inline-code-end]

---