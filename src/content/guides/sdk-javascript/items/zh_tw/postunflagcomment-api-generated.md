## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'postUnFlagComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "comment_98765";
  const broadcastId: string = "broadcast_55555";
  const sso: string = "sso_token_abcde";

  const result1: APIEmptyResponse = await postUnFlagComment(tenantId, commentId);
  const result2: APIEmptyResponse = await postUnFlagComment(tenantId, commentId, broadcastId, sso);
}

runExample();
[inline-code-end]