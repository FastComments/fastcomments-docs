req
tenantId
urlId

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| page | number | לא |  |
| direction | SortDirections | לא |  |
| sso | string | לא |  |
| skip | number | לא |  |
| skipChildren | number | לא |  |
| limit | number | לא |  |
| limitChildren | number | לא |  |
| countChildren | boolean | לא |  |
| fetchPageForCommentId | string | לא |  |
| includeConfig | boolean | לא |  |
| countAll | boolean | לא |  |
| includei10n | boolean | לא |  |
| locale | string | לא |  |
| modules | string | לא |  |
| isCrawler | boolean | לא |  |
| includeNotificationCount | boolean | לא |  |
| asTree | boolean | לא |  |
| maxTreeDepth | number | לא |  |
| useFullTranslationIds | boolean | לא |  |
| parentId | string | לא |  |
| searchText | string | לא |  |
| hashTags | Array<string> | לא |  |
| userId | string | לא |  |
| customConfigStr | string | לא |  |
| afterCommentId | string | לא |  |
| beforeCommentId | string | לא |  |

## תגובה

מחזיר: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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