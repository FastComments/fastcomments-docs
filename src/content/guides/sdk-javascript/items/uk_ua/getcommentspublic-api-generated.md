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