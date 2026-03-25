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

Возвращает: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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