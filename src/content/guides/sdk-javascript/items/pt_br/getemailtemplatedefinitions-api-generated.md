## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |

## Resposta

Retorna: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_001';
  const options: { includeDrafts?: boolean } = { includeDrafts: true }; // parâmetro opcional demonstrado
  const templates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId, options);
  console.log(templates);
})();
[inline-code-end]

---