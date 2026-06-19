req
tenantId
urlId

## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|------|------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| page | number | いいえ |  |
| direction | SortDirections | いいえ |  |
| sso | string | いいえ |  |
| skip | number | いいえ |  |
| skipChildren | number | いいえ |  |
| limit | number | いいえ |  |
| limitChildren | number | いいえ |  |
| countChildren | boolean | いいえ |  |
| fetchPageForCommentId | string | いいえ |  |
| includeConfig | boolean | いいえ |  |
| countAll | boolean | いいえ |  |
| includei10n | boolean | いいえ |  |
| locale | string | いいえ |  |
| modules | string | いいえ |  |
| isCrawler | boolean | いいえ |  |
| includeNotificationCount | boolean | いいえ |  |
| asTree | boolean | いいえ |  |
| maxTreeDepth | number | いいえ |  |
| useFullTranslationIds | boolean | いいえ |  |
| parentId | string | いいえ |  |
| searchText | string | いいえ |  |
| hashTags | Array<string> | いいえ |  |
| userId | string | いいえ |  |
| customConfigStr | string | いいえ |  |
| afterCommentId | string | いいえ |  |
| beforeCommentId | string | いいえ |  |

## レスポンス

戻り値: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## 例

[inline-code-attrs-start title = 'getCommentsPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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