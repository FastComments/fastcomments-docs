req
tenantId
urlId

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nee |  |
| direction | SortDirections | Nee |  |
| sso | string | Nee |  |
| skip | number | Nee |  |
| skipChildren | number | Nee |  |
| limit | number | Nee |  |
| limitChildren | number | Nee |  |
| countChildren | boolean | Nee |  |
| fetchPageForCommentId | string | Nee |  |
| includeConfig | boolean | Nee |  |
| countAll | boolean | Nee |  |
| includei10n | boolean | Nee |  |
| locale | string | Nee |  |
| modules | string | Nee |  |
| isCrawler | boolean | Nee |  |
| includeNotificationCount | boolean | Nee |  |
| asTree | boolean | Nee |  |
| maxTreeDepth | number | Nee |  |
| useFullTranslationIds | boolean | Nee |  |
| parentId | string | Nee |  |
| searchText | string | Nee |  |
| hashTags | Array<string> | Nee |  |
| userId | string | Nee |  |
| customConfigStr | string | Nee |  |
| afterCommentId | string | Nee |  |
| beforeCommentId | string | Nee |  |

## Respons

Retourneert: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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