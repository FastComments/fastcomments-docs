req
tenantId
urlId

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| page | number | Ne |  |
| direction | SortDirections | Ne |  |
| sso | string | Ne |  |
| skip | number | Ne |  |
| skipChildren | number | Ne |  |
| limit | number | Ne |  |
| limitChildren | number | Ne |  |
| countChildren | boolean | Ne |  |
| fetchPageForCommentId | string | Ne |  |
| includeConfig | boolean | Ne |  |
| countAll | boolean | Ne |  |
| includei10n | boolean | Ne |  |
| locale | string | Ne |  |
| modules | string | Ne |  |
| isCrawler | boolean | Ne |  |
| includeNotificationCount | boolean | Ne |  |
| asTree | boolean | Ne |  |
| maxTreeDepth | number | Ne |  |
| useFullTranslationIds | boolean | Ne |  |
| parentId | string | Ne |  |
| searchText | string | Ne |  |
| hashTags | Array<string> | Ne |  |
| userId | string | Ne |  |
| customConfigStr | string | Ne |  |
| afterCommentId | string | Ne |  |
| beforeCommentId | string | Ne |  |

## Одговор

Vraća: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

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