## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| tenantId | string | No |  |
| sso | string | No |  |

## レスポンス

戻り値: [`GetLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetLogsResponse.ts)

## 例

[inline-code-attrs-start title = 'getLogs の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchLogs() {
    const commentId: string = "cmt_9a8b7c6d5e4f3a2b";
    const tenantId: string = "tenant_9876";
    const sso: string = "sso_abcdef123456";

    const fullResponse: GetLogsResponse = await getLogs(commentId, tenantId, sso);
    const minimalResponse: GetLogsResponse = await getLogs(commentId);
}
[inline-code-end]