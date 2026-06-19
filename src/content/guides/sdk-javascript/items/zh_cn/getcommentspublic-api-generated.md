req
tenantId
urlId

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| page | number | 否 |  |
| direction | SortDirections | 否 |  |
| sso | string | 否 |  |
| skip | number | 否 |  |
| skipChildren | number | 否 |  |
| limit | number | 否 |  |
| limitChildren | number | 否 |  |
| countChildren | boolean | 否 |  |
| fetchPageForCommentId | string | 否 |  |
| includeConfig | boolean | 否 |  |
| countAll | boolean | 否 |  |
| includei10n | boolean | 否 |  |
| locale | string | 否 |  |
| modules | string | 否 |  |
| isCrawler | boolean | 否 |  |
| includeNotificationCount | boolean | 否 |  |
| asTree | boolean | 否 |  |
| maxTreeDepth | number | 否 |  |
| useFullTranslationIds | boolean | 否 |  |
| parentId | string | 否 |  |
| searchText | string | 否 |  |
| hashTags | Array<string> | 否 |  |
| userId | string | 否 |  |
| customConfigStr | string | 否 |  |
| afterCommentId | string | 否 |  |
| beforeCommentId | string | 否 |  |

## 响应

返回: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## 示例

[inline-code-attrs-start title = 'getCommentsPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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