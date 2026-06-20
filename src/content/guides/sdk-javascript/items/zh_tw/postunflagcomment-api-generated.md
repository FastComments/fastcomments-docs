---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'postUnFlagComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = '5f8d04b2-9c3a-4d13-bb8a-123456789abc';
  const resultWithoutSso: APIEmptyResponse = await postUnFlagComment(commentId);
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0NTY3OCJ9.signature';
  const resultWithSso: APIEmptyResponse = await postUnFlagComment(commentId, ssoToken);
  console.log(resultWithoutSso, resultWithSso);
})();
[inline-code-end]

---