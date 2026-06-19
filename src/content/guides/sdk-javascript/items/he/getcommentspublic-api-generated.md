req
tenantId
urlId

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| page | number | לא |  |
| direction | SortDirections | לא |  |
| sso | string | לא |  |
| skip | number | לא |  |
| skipChildren | number | לא |  |
| limit | number | לא |  |
| limitChildren | number | לא |  |
| countChildren | boolean | לא |  |
| fetchPageForCommentId | string | לא |  |
| includeConfig | boolean | לא |  |
| countAll | boolean | לא |  |
| includei10n | boolean | לא |  |
| locale | string | לא |  |
| modules | string | לא |  |
| isCrawler | boolean | לא |  |
| includeNotificationCount | boolean | לא |  |
| asTree | boolean | לא |  |
| maxTreeDepth | number | לא |  |
| useFullTranslationIds | boolean | לא |  |
| parentId | string | לא |  |
| searchText | string | לא |  |
| hashTags | Array<string> | לא |  |
| userId | string | לא |  |
| customConfigStr | string | לא |  |
| afterCommentId | string | לא |  |
| beforeCommentId | string | לא |  |

## תגובה

מחזיר: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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