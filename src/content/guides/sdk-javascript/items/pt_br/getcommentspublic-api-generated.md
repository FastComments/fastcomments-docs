req
tenantId
urlId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
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

[inline-code-attrs-start title = 'Exemplo getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments(): Promise<void> {
  const tenantId: string = "tenant-42",
        urlId: string = "post-2023-09-15",
        page: number = 1,
        direction: SortDirections = "desc",
        limit: number = 30,
        includeConfig: boolean = true,
        locale: string = "en-US",
        isCrawler: boolean = false,
        asTree: boolean = true,
        maxTreeDepth: number = 2,
        searchText: string = "fastcomments",
        hashTags: string[] = ["fastcomments","typescript"],
        userId: string = "user-123";

  const result: GetCommentsResponseWithPresencePublicComment = await getCommentsPublic(
    tenantId,
    urlId,
    page,
    direction,
    undefined,
    undefined,
    undefined,
    limit,
    undefined,
    undefined,
    undefined,
    includeConfig,
    undefined,
    undefined,
    locale,
    undefined,
    isCrawler,
    undefined,
    asTree,
    maxTreeDepth,
    undefined,
    undefined,
    searchText,
    hashTags,
    userId
  );
}
[inline-code-end]