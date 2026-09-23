## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## 응답

반환: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## 예제

[inline-code-attrs-start title = 'unPinComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-20230915-001";
  const broadcastId: string = "broadcast-2023-09-15";
  const ssoToken: string = "sso-abc123";

  const result: ChangeCommentPinStatusResponse = await unPinComment(tenantId, commentId, broadcastId, ssoToken);
  const resultWithoutSso: ChangeCommentPinStatusResponse = await unPinComment(tenantId, commentId, broadcastId);
})();
[inline-code-end]