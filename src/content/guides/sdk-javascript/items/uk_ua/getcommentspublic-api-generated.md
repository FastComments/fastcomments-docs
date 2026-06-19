req
tenantId
urlId

## Параметри

| Назва | Тип | Обов'язковий | Опис |
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

Повертає: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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