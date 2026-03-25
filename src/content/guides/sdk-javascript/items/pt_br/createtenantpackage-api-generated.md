## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| createTenantPackageBody | CreateTenantPackageBody | Sim |  |

## Resposta

Retorna: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: 'Standard Moderation',
  description: 'Suitable for small-to-medium sites: basic moderation, spam rules, and analytics',
  maxCommentsPerMinute: 50,
  allowAnonymousComments: false, // parâmetro opcional fornecido
  // campos opcionais omitidos: por exemplo, regras de moderação avançadas, CSS personalizado
  customConfigParameters: {
    enableProfanityFilter: true,
    imageContentProfanityLevel: 'medium' // valor ilustrativo; usa a forma CustomConfigParameters
  }
};
const response: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
console.log(response);
[inline-code-end]