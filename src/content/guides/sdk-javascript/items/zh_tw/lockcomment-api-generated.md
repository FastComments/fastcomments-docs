## Parameters

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## Response

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'lockComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoLockComment() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_9876";
  const broadcastId: string = "brd_5555";
  const ssoToken: string = "sso_user_abc";

  const resultWithSso: APIEmptyResponse = await lockComment(tenantId, commentId, broadcastId, ssoToken);
  const resultWithoutSso: APIEmptyResponse = await lockComment(tenantId, commentId, broadcastId);
}

demoLockComment();
[inline-code-end]

---