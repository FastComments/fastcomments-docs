req
tenantId
afterId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| afterId | string | Não |  |
| limit | number | Não |  |
| tags | Array<string> | Não |  |
| sso | string | Não |  |
| isCrawler | boolean | Não |  |
| includeUserInfo | boolean | Não |  |

## Resposta

Retorna: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getFeedPostsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_01';
  const afterId: string = 'post_20250610_842';
  const limit: number = 25;
  const tags: string[] = ['news', 'technology'];
  const sso: string = 'sso_jwt_eyJhbGciOiJIUzI1Ni';
  const isCrawler: boolean = false;
  const includeUserInfo: boolean = true;

  const response: GetFeedPostsPublic200Response = await getFeedPostsPublic(
    tenantId,
    afterId,
    limit,
    tags,
    sso,
    isCrawler,
    includeUserInfo
  );

  console.log(response);
})();
[inline-code-end]

---