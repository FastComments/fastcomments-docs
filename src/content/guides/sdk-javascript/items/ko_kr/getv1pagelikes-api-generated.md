## 매개변수

| 이름 | 타입 | 필수 여부 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| sso | string | 아니요 |  |
| includeUserInfo | PageReactIncludeUserInfo | 아니요 |  |

## 응답

반환값: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes.ts)

## 예제

[inline-code-attrs-start title = 'getV1PageLikes 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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