req
tenantId
urlId

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| page | number | Ne |  |
| direction | SortDirections | Ne |  |
| sso | string | Ne |  |
| skip | number | Ne |  |
| skipChildren | number | Ne |  |
| limit | number | Ne |  |
| limitChildren | number | Ne |  |
| countChildren | boolean | Ne |  |
| fetchPageForCommentId | string | Ne |  |
| includeConfig | boolean | Ne |  |
| countAll | boolean | Ne |  |
| includei10n | boolean | Ne |  |
| locale | string | Ne |  |
| modules | string | Ne |  |
| isCrawler | boolean | Ne |  |
| includeNotificationCount | boolean | Ne |  |
| asTree | boolean | Ne |  |
| maxTreeDepth | number | Ne |  |
| useFullTranslationIds | boolean | Ne |  |
| parentId | string | Ne |  |
| searchText | string | Ne |  |
| hashTags | Array<string> | Ne |  |
| userId | string | Ne |  |
| customConfigStr | string | Ne |  |
| afterCommentId | string | Ne |  |
| beforeCommentId | string | Ne |  |

## Odgovor

Vraća: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Primjer

[inline-code-attrs-start title = 'getCommentsPublic Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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