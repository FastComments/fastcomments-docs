req
tenantId
urlId

## Параметри

| Име | Тип | Обавезно | Опис |
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
const tenantId: string = 'tenant_eu-west_01';
const urlId: string = 'https://www.financialtimes.com/articles/2026/market-update-q1';
const response: GetCommentsPublic200Response = await getCommentsPublic(
  tenantId,
  urlId,
  2,
  undefined,
  'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.tokenPayload.signature',
  undefined,
  0,
  50,
  5,
  true,
  undefined,
  true,
  false,
  true,
  'en-US',
  'reactions,moderation',
  false,
  true,
  true,
  3,
  false,
  undefined,
  'performance',
  ['feature','fastcomments'],
  'user_9876',
  undefined,
  undefined,
  undefined
);
[inline-code-end]