req
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

Връща: [`GetCommentsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublicResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = 'acme-corp';
  const urlId: string = 'blog/post-789';
  const page: number = 1;
  const direction: SortDirections = SortDirections.Desc;
  const limit: number = 25;
  const includeConfig: boolean = true;
  const locale: string = 'en-GB';
  const modules: string = 'reactions,attachments';
  const isCrawler: boolean = false;
  const includeNotificationCount: boolean = true;
  const asTree: boolean = true;
  const maxTreeDepth: number = 4;
  const searchText: string = 'TypeScript';
  const hashTags: string[] = ['typescript', 'api'];
  const response: GetCommentsPublicResponse = await getCommentsPublic(
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
    modules,
    isCrawler,
    includeNotificationCount,
    asTree,
    maxTreeDepth,
    undefined,
    undefined,
    searchText,
    hashTags,
    undefined,
    undefined,
    undefined,
    undefined
  );
}
[inline-code-end]