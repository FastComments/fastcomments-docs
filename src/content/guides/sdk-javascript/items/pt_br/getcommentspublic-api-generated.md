req
tenantId
urlId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| page | number | Não |  |
| direction | SortDirections | Não |  |
| sso | string | Não |  |
| skip | number | Não |  |
| skipChildren | number | Não |  |
| limit | number | Não |  |
| limitChildren | number | Não |  |
| countChildren | boolean | Não |  |
| fetchPageForCommentId | string | Não |  |
| includeConfig | boolean | Não |  |
| countAll | boolean | Não |  |
| includei10n | boolean | Não |  |
| locale | string | Não |  |
| modules | string | Não |  |
| isCrawler | boolean | Não |  |
| includeNotificationCount | boolean | Não |  |
| asTree | boolean | Não |  |
| maxTreeDepth | number | Não |  |
| useFullTranslationIds | boolean | Não |  |
| parentId | string | Não |  |
| searchText | string | Não |  |
| hashTags | Array<string> | Não |  |
| userId | string | Não |  |
| customConfigStr | string | Não |  |
| afterCommentId | string | Não |  |
| beforeCommentId | string | Não |  |

## Resposta

Retorna: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const response: GetCommentsResponseWithPresencePublicComment = await getCommentsPublic(
    'news-tenant-42',
    'article-2026-06-19-abc123',
    1,
    undefined,
    'sso_eyJhbGciOiJIUzI1Ni',
    0,
    0,
    25,
    5,
    true,
    undefined,
    true,
    false,
    true,
    'en-US',
    'reactions,moderation',
    false,
    true,
    true,
    3,
    false,
    undefined,
    'climate change',
    ['environment', 'policy'],
    'user-789',
    undefined,
    undefined,
    undefined
  );
  console.log(response);
})();
[inline-code-end]