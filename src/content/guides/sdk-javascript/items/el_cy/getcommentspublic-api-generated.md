req
tenantId
urlId

## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| page | number | Όχι |  |
| direction | SortDirections | Όχι |  |
| sso | string | Όχι |  |
| skip | number | Όχι |  |
| skipChildren | number | Όχι |  |
| limit | number | Όχι |  |
| limitChildren | number | Όχι |  |
| countChildren | boolean | Όχι |  |
| fetchPageForCommentId | string | Όχι |  |
| includeConfig | boolean | Όχι |  |
| countAll | boolean | Όχι |  |
| includei10n | boolean | Όχι |  |
| locale | string | Όχι |  |
| modules | string | Όχι |  |
| isCrawler | boolean | Όχι |  |
| includeNotificationCount | boolean | Όχι |  |
| asTree | boolean | Όχι |  |
| maxTreeDepth | number | Όχι |  |
| useFullTranslationIds | boolean | Όχι |  |
| parentId | string | Όχι |  |
| searchText | string | Όχι |  |
| hashTags | Array<string> | Όχι |  |
| userId | string | Όχι |  |
| customConfigStr | string | Όχι |  |
| afterCommentId | string | Όχι |  |
| beforeCommentId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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