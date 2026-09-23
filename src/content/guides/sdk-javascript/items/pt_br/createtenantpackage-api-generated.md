## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| createTenantPackageBody | CreateTenantPackageBody | Sim |  |

## Resposta

Retorna: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9f8b7c6d";
  const createTenantPackageBody: CreateTenantPackageBody = {
    name: "Enterprise Package",
    priceCents: 49999,
    // campo opcional
    description: "Full suite of enterprise features"
  };
  const result: CreateTenantPackageResponse = await createTenantPackage(tenantId, createTenantPackageBody);
  console.log(result);
}
runExample();
[inline-code-end]

---