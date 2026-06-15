## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## 範例

[inline-code-attrs-start title = 'unFlagComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2a1f';
const commentId: string = 'cmt_20250614_01';
const userId: string = 'user_47d2b9';
const result: FlagComment200Response = await unFlagComment(tenantId, commentId, userId);
[inline-code-end]

---