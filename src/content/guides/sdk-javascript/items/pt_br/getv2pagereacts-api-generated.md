---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |

## Resposta

Retorna: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getV2PageReacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-82';
const urlId: string = 'https://www.acmecorp.com/blog/product-launch-2026';
const reacts: GetV2PageReacts = await getV2PageReacts(tenantId, urlId);
console.log(reacts);
[inline-code-end]

---