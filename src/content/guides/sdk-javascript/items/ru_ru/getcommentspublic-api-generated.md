req
tenantId
urlId

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| page | number | Нет |  |
| direction | SortDirections | Нет |  |
| sso | string | Нет |  |
| skip | number | Нет |  |
| skipChildren | number | Нет |  |
| limit | number | Нет |  |
| limitChildren | number | Нет |  |
| countChildren | boolean | Нет |  |
| fetchPageForCommentId | string | Нет |  |
| includeConfig | boolean | Нет |  |
| countAll | boolean | Нет |  |
| includei10n | boolean | Нет |  |
| locale | string | Нет |  |
| modules | string | Нет |  |
| isCrawler | boolean | Нет |  |
| includeNotificationCount | boolean | Нет |  |
| asTree | boolean | Нет |  |
| maxTreeDepth | number | Нет |  |
| useFullTranslationIds | boolean | Нет |  |
| parentId | string | Нет |  |
| searchText | string | Нет |  |
| hashTags | Array<string> | Нет |  |
| userId | string | Нет |  |
| customConfigStr | string | Нет |  |
| afterCommentId | string | Нет |  |
| beforeCommentId | string | Нет |  |

## Ответ

Возвращает: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Пример

[inline-code-attrs-start title = 'getCommentsPublic Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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