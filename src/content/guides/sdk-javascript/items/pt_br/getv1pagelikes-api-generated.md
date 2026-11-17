## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| sso | string | Não |  |
| includeUserInfo | PageReactIncludeUserInfo | Não |  |

## Resposta

Retorna: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getV1PageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'https://example.com/blog/my-post';

const result: GetV1PageLikes = await getV1PageLikes(tenantId, urlId, 'sso-token-xyz789', true);

console.log(result.status);
if (result.likes) {
  result.likes.forEach((like) => {
    console.log(like.userId, like.date);
  });
}
[inline-code-end]