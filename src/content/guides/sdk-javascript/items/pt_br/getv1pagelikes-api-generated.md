## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |

## Resposta

Retorna: [`GetV1PageLikes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getV1PageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-742';
const urlId: string = 'sports/2026/06/15/world-cup-preview';
const pageLikes: GetV1PageLikes200Response = await getV1PageLikes(tenantId, urlId);
[inline-code-end]

---