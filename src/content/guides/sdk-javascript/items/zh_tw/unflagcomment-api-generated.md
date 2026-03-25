## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳： [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## 範例

[inline-code-attrs-start title = 'unFlagComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const commentId: string = 'cmt_9f8e7d6c';
const userId: string = 'user_72b4a1c9';
const anonUserId: string = 'anon_3d2c1b0a';
const response: FlagComment200Response = await unFlagComment(tenantId, commentId, userId, anonUserId);
[inline-code-end]

---