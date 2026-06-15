req
tenantId
urlId

## Параметри

| Назив | Тип | Обавезно | Опис |
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

## Одговор

Враћа: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'getCommentsPublic Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-news';
const urlId: string = '/articles/2026/fastcomments-update';
const page: number = 1;
const skip: number = 0;
const limit: number = 25;
const countChildren: boolean = true;
const includeConfig: boolean = true;
const result: GetCommentsPublic200Response = await getCommentsPublic(
  tenantId,
  urlId,
  page,
  undefined,
  undefined,
  skip,
  undefined,
  limit,
  undefined,
  countChildren,
  undefined,
  includeConfig
);
[inline-code-end]