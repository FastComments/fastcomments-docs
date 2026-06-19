req
tenantId
urlId

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nej |  |
| direction | SortDirections | Nej |  |
| sso | string | Nej |  |
| skip | number | Nej |  |
| skipChildren | number | Nej |  |
| limit | number | Nej |  |
| limitChildren | number | Nej |  |
| countChildren | boolean | Nej |  |
| fetchPageForCommentId | string | Nej |  |
| includeConfig | boolean | Nej |  |
| countAll | boolean | Nej |  |
| includei10n | boolean | Nej |  |
| locale | string | Nej |  |
| modules | string | Nej |  |
| isCrawler | boolean | Nej |  |
| includeNotificationCount | boolean | Nej |  |
| asTree | boolean | Nej |  |
| maxTreeDepth | number | Nej |  |
| useFullTranslationIds | boolean | Nej |  |
| parentId | string | Nej |  |
| searchText | string | Nej |  |
| hashTags | Array<string> | Nej |  |
| userId | string | Nej |  |
| customConfigStr | string | Nej |  |
| afterCommentId | string | Nej |  |
| beforeCommentId | string | Nej |  |

## Svar

Returnerer: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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