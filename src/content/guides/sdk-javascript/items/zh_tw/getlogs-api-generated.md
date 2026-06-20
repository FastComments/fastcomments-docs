## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetLogsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getLogs 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = 'cmt_0f9b1a2c3d4e5f6a';
  const logsWithoutSSO: ModerationAPIGetLogsResponse = await getLogs(commentId);
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1Njc4OSJ9.dQw4w9WgXcQ';
  const logsWithSSO: ModerationAPIGetLogsResponse = await getLogs(commentId, ssoToken);
  console.log(logsWithoutSSO, logsWithSSO);
})();
[inline-code-end]

---