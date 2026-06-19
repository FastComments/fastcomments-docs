req
tenantId
afterId

## Parâmetros

| Name | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| afterId | string | Não |  |
| limit | number | Não |  |
| tags | Array<string> | Não |  |
| sso | string | Não |  |
| isCrawler | boolean | Não |  |
| includeUserInfo | boolean | Não |  |

## Resposta

Retorna: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicFeedPostsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'getFeedPostsPublic Exemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "fc_tenant_12345";
  const afterId: string = "feedPost_98765";
  const limit: number = 20;
  const tags: Array<string> = ["announcement", "product"];
  const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example";
  const isCrawler: boolean = false;
  const includeUserInfo: boolean = true;

  const response: PublicFeedPostsResponse = await getFeedPostsPublic(tenantId, afterId, limit, tags, sso, isCrawler, includeUserInfo);
  console.log(response);
})();
[inline-code-end]

---