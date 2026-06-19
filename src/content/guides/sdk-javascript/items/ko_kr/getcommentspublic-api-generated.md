req
tenantId
urlId

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| page | number | 아니오 |  |
| direction | SortDirections | 아니오 |  |
| sso | string | 아니오 |  |
| skip | number | 아니오 |  |
| skipChildren | number | 아니오 |  |
| limit | number | 아니오 |  |
| limitChildren | number | 아니오 |  |
| countChildren | boolean | 아니오 |  |
| fetchPageForCommentId | string | 아니오 |  |
| includeConfig | boolean | 아니오 |  |
| countAll | boolean | 아니오 |  |
| includei10n | boolean | 아니오 |  |
| locale | string | 아니오 |  |
| modules | string | 아니오 |  |
| isCrawler | boolean | 아니오 |  |
| includeNotificationCount | boolean | 아니오 |  |
| asTree | boolean | 아니오 |  |
| maxTreeDepth | number | 아니오 |  |
| useFullTranslationIds | boolean | 아니오 |  |
| parentId | string | 아니오 |  |
| searchText | string | 아니오 |  |
| hashTags | Array<string> | 아니오 |  |
| userId | string | 아니오 |  |
| customConfigStr | string | 아니오 |  |
| afterCommentId | string | 아니오 |  |
| beforeCommentId | string | 아니오 |  |

## 응답

반환: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## 예제

[inline-code-attrs-start title = 'getCommentsPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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