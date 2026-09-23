req
tenantId
urlId

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| page | number | No |  |
| direction | SortDirections | No |  |
| sso | string | No |  |
| skip | number | No |  |
| skipChildren | number | No |  |
| limit | number | No |  |
| limitChildren | number | No |  |
| countChildren | boolean | No |  |
| fetchPageForCommentId | string | No |  |
| includeConfig | boolean | No |  |
| countAll | boolean | No |  |
| includei10n | boolean | No |  |
| locale | string | No |  |
| modules | string | No |  |
| isCrawler | boolean | No |  |
| includeNotificationCount | boolean | No |  |
| asTree | boolean | No |  |
| maxTreeDepth | number | No |  |
| useFullTranslationIds | boolean | No |  |
| parentId | string | No |  |
| searchText | string | No |  |
| hashTags | Array<string> | No |  |
| userId | string | No |  |
| customConfigStr | string | No |  |
| afterCommentId | string | No |  |
| beforeCommentId | string | No |  |

## Odgovor

Vrne: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Primer

[inline-code-attrs-start title = 'Primer getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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