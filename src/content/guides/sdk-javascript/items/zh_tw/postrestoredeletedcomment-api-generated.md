## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'postRestoreDeletedComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = 'cmt_7f3b2a1e-54d3';
  const resultWithoutSso: APIEmptyResponse = await postRestoreDeletedComment(commentId);
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4OSIsImlhdCI6MTYyMzQ1Njc4OX0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
  const resultWithSso: APIEmptyResponse = await postRestoreDeletedComment(commentId, ssoToken);
  console.log(resultWithoutSso, resultWithSso);
})();
[inline-code-end]

---