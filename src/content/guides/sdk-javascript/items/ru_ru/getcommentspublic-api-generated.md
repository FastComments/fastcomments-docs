req
tenantId
urlId

## Параметры

| Name | Тип | Обязательно | Описание |
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

Возвращает: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---