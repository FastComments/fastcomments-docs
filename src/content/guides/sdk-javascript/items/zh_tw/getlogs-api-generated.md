## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## 回應

返回：[`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetLogsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getLogs 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const commentId: string = "cmt_4567890";
  const ssoToken: string = "sso_abcdef123456";

  const logsWithSso: ModerationAPIGetLogsResponse = await getLogs(tenantId, commentId, ssoToken);
  const logsWithoutSso: ModerationAPIGetLogsResponse = await getLogs(tenantId, commentId);
})();
[inline-code-end]