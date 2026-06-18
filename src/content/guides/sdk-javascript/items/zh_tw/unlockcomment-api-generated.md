## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`LockComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockComment200Response.ts)

## 範例

[inline-code-attrs-start title = 'unLockComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-8f3b2c4a";
const commentId: string = "cmt_92a7f3e6";
const broadcastId: string = "brd_1b4c9d20";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const result: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---