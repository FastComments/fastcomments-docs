req
tenantId
urlId

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| page | number | Ні |  |
| direction | SortDirections | Ні |  |
| sso | string | Ні |  |
| skip | number | Ні |  |
| skipChildren | number | Ні |  |
| limit | number | Ні |  |
| limitChildren | number | Ні |  |
| countChildren | boolean | Ні |  |
| fetchPageForCommentId | string | Ні |  |
| includeConfig | boolean | Ні |  |
| countAll | boolean | Ні |  |
| includei10n | boolean | Ні |  |
| locale | string | Ні |  |
| modules | string | Ні |  |
| isCrawler | boolean | Ні |  |
| includeNotificationCount | boolean | Ні |  |
| asTree | boolean | Ні |  |
| maxTreeDepth | number | Ні |  |
| useFullTranslationIds | boolean | Ні |  |
| parentId | string | Ні |  |
| searchText | string | Ні |  |
| hashTags | Array<string> | Ні |  |
| userId | string | Ні |  |
| customConfigStr | string | Ні |  |
| afterCommentId | string | Ні |  |
| beforeCommentId | string | Ні |  |

## Відповідь

Повертає: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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