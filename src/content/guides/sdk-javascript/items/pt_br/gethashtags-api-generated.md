## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| page | number | Não |  |

## Resposta

Retorna: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTags200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getHashTags'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const pageNumber: number = 2;
const responseWithPage: GetHashTags200Response = await getHashTags(tenantId, pageNumber);
const responseWithoutPage: GetHashTags200Response = await getHashTags(tenantId);
[inline-code-end]

---