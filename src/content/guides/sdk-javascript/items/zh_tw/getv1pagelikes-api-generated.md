## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| sso | string | 否 |  |
| includeUserInfo | PageReactIncludeUserInfo | 否 |  |

## 回應

回傳:[`GetV1PageLikes`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes.ts)

## 範例

[inline-code-attrs-start title = 'getV1PageLikes 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'https://example.com/blog/my-post';

const result: GetV1PageLikes = await getV1PageLikes(tenantId, urlId, 'sso-token-xyz789', true);

console.log(result.status);
if (result.likes) {
  result.likes.forEach((like) => {
    console.log(like.userId, like.date);
  });
}
[inline-code-end]