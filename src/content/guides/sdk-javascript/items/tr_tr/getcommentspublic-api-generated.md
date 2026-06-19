req
tenantId
urlId

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| page | number | Hayır |  |
| direction | SortDirections | Hayır |  |
| sso | string | Hayır |  |
| skip | number | Hayır |  |
| skipChildren | number | Hayır |  |
| limit | number | Hayır |  |
| limitChildren | number | Hayır |  |
| countChildren | boolean | Hayır |  |
| fetchPageForCommentId | string | Hayır |  |
| includeConfig | boolean | Hayır |  |
| countAll | boolean | Hayır |  |
| includei10n | boolean | Hayır |  |
| locale | string | Hayır |  |
| modules | string | Hayır |  |
| isCrawler | boolean | Hayır |  |
| includeNotificationCount | boolean | Hayır |  |
| asTree | boolean | Hayır |  |
| maxTreeDepth | number | Hayır |  |
| useFullTranslationIds | boolean | Hayır |  |
| parentId | string | Hayır |  |
| searchText | string | Hayır |  |
| hashTags | Array<string> | Hayır |  |
| userId | string | Hayır |  |
| customConfigStr | string | Hayır |  |
| afterCommentId | string | Hayır |  |
| beforeCommentId | string | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Örnek

[inline-code-attrs-start title = 'getCommentsPublic Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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