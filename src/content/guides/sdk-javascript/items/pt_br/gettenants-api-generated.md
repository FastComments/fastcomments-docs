## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| meta | string | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTenants'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_9f2d1b7c';
  const meta: string = 'include=domains,billing,customConfig';
  const skip: number = 20;
  const response: GetTenants200Response = await getTenants(tenantId, meta, skip);
  console.log(response);
})();
[inline-code-end]

---