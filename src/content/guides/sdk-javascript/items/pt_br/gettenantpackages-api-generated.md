## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetTenantPackagesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse1.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getTenantPackages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_67890';
  const skip: number = 30;

  const packagesWithSkip: GetTenantPackagesResponse1 = await getTenantPackages(tenantId, skip);
  const packagesWithoutSkip: GetTenantPackagesResponse1 = await getTenantPackages(tenantId);

  console.log(packagesWithSkip, packagesWithoutSkip);
})();
[inline-code-end]