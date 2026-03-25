## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |

## Odgovor

Vraća: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_eu_01';
const templates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId);
console.log('Email template definitions loaded for', tenantId, templates);
[inline-code-end]

---