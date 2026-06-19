req
tenantId
urlId

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| page | number | Non |  |
| direction | SortDirections | Non |  |
| sso | string | Non |  |
| skip | number | Non |  |
| skipChildren | number | Non |  |
| limit | number | Non |  |
| limitChildren | number | Non |  |
| countChildren | boolean | Non |  |
| fetchPageForCommentId | string | Non |  |
| includeConfig | boolean | Non |  |
| countAll | boolean | Non |  |
| includei10n | boolean | Non |  |
| locale | string | Non |  |
| modules | string | Non |  |
| isCrawler | boolean | Non |  |
| includeNotificationCount | boolean | Non |  |
| asTree | boolean | Non |  |
| maxTreeDepth | number | Non |  |
| useFullTranslationIds | boolean | Non |  |
| parentId | string | Non |  |
| searchText | string | Non |  |
| hashTags | Array<string> | Non |  |
| userId | string | Non |  |
| customConfigStr | string | Non |  |
| afterCommentId | string | Non |  |
| beforeCommentId | string | Non |  |

## Réponse

Renvoie: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'utilisation de getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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