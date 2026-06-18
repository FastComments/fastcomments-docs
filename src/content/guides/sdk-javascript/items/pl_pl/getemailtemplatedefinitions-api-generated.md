---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |

## Odpowiedź

Zwraca: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_001';
  const options: { includeDrafts?: boolean } = { includeDrafts: true }; // pokazuje użycie opcjonalnego parametru
  const templates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId, options);
  console.log(templates);
})();
[inline-code-end]

---