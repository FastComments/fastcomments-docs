req
tenantId
urlId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nee |  |
| direction | SortDirections | Nee |  |
| sso | string | Nee |  |
| skip | number | Nee |  |
| skipChildren | number | Nee |  |
| limit | number | Nee |  |
| limitChildren | number | Nee |  |
| countChildren | boolean | Nee |  |
| fetchPageForCommentId | string | Nee |  |
| includeConfig | boolean | Nee |  |
| countAll | boolean | Nee |  |
| includei10n | boolean | Nee |  |
| locale | string | Nee |  |
| modules | string | Nee |  |
| isCrawler | boolean | Nee |  |
| includeNotificationCount | boolean | Nee |  |
| asTree | boolean | Nee |  |
| maxTreeDepth | number | Nee |  |
| useFullTranslationIds | boolean | Nee |  |
| parentId | string | Nee |  |
| searchText | string | Nee |  |
| hashTags | Array<string> | Nee |  |
| userId | string | Nee |  |
| customConfigStr | string | Nee |  |
| afterCommentId | string | Nee |  |
| beforeCommentId | string | Nee |  |

## Response

Returns: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Example

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---