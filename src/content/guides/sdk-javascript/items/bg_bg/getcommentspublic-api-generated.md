заявка
tenantId
urlId

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| page | number | Не |  |
| direction | SortDirections | Не |  |
| sso | string | Не |  |
| skip | number | Не |  |
| skipChildren | number | Не |  |
| limit | number | Не |  |
| limitChildren | number | Не |  |
| countChildren | boolean | Не |  |
| fetchPageForCommentId | string | Не |  |
| includeConfig | boolean | Не |  |
| countAll | boolean | Не |  |
| includei10n | boolean | Не |  |
| locale | string | Не |  |
| modules | string | Не |  |
| isCrawler | boolean | Не |  |
| includeNotificationCount | boolean | Не |  |
| asTree | boolean | Не |  |
| maxTreeDepth | number | Не |  |
| useFullTranslationIds | boolean | Не |  |
| parentId | string | Не |  |
| searchText | string | Не |  |
| hashTags | Array<string> | Не |  |
| userId | string | Не |  |
| customConfigStr | string | Не |  |
| afterCommentId | string | Не |  |
| beforeCommentId | string | Не |  |

## Отговор

Връща: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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