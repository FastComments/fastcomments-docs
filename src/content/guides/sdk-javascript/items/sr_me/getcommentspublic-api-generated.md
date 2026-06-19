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

Враћа: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Пример

[inline-code-attrs-start title = 'getCommentsPublic Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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