## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse.ts)

## 示例

[inline-code-attrs-start title = 'getCommentBanStatus 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = 'd3b07384-d9f1-4b3a-9f82-1a2b3c4d5e6f';
  const banStatusDefault: GetCommentBanStatusResponse = await getCommentBanStatus(commentId);
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0MjM0IiwiaWF0IjoxNjMwMDAwMDB9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
  const banStatusWithSso: GetCommentBanStatusResponse = await getCommentBanStatus(commentId, ssoToken);
  console.log(banStatusDefault, banStatusWithSso);
})();
[inline-code-end]

---