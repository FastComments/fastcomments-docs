req
tenantId
urlId

## Параметры

| Name | Type | Required | Description |
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

[inline-code-attrs-start title = 'Пример getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const response: GetCommentsResponseWithPresencePublicComment = await getCommentsPublic(
    'news-tenant-42',
    'article-2026-06-19-abc123',
    1,
    undefined,
    'sso_eyJhbGciOiJIUzI1Ni',
    0,
    0,
    25,
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
    'climate change',
    ['environment', 'policy'],
    'user-789',
    undefined,
    undefined,
    undefined
  );
  console.log(response);
})();
[inline-code-end]