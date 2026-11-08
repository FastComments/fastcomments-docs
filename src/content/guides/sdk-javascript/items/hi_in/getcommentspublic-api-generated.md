req
tenantId
urlId

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| page | number | नहीं |  |
| direction | SortDirections | नहीं |  |
| sso | string | नहीं |  |
| skip | number | नहीं |  |
| skipChildren | number | नहीं |  |
| limit | number | नहीं |  |
| limitChildren | number | नहीं |  |
| countChildren | boolean | नहीं |  |
| fetchPageForCommentId | string | नहीं |  |
| includeConfig | boolean | नहीं |  |
| countAll | boolean | नहीं |  |
| includei10n | boolean | नहीं |  |
| locale | string | नहीं |  |
| modules | string | नहीं |  |
| isCrawler | boolean | नहीं |  |
| includeNotificationCount | boolean | नहीं |  |
| asTree | boolean | नहीं |  |
| maxTreeDepth | number | नहीं |  |
| useFullTranslationIds | boolean | नहीं |  |
| parentId | string | नहीं |  |
| searchText | string | नहीं |  |
| hashTags | Array<string> | नहीं |  |
| userId | string | नहीं |  |
| customConfigStr | string | नहीं |  |
| afterCommentId | string | नहीं |  |
| beforeCommentId | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## उदाहरण

[inline-code-attrs-start title = 'getCommentsPublic उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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