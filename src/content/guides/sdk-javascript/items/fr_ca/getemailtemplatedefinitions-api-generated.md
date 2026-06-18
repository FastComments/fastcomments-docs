## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |

## Réponse

Renvoie : [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Exemple

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_001';
  const options: { includeDrafts?: boolean } = { includeDrafts: true }; // paramètre optionnel démontré
  const templates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId, options);
  console.log(templates);
})();
[inline-code-end]

---