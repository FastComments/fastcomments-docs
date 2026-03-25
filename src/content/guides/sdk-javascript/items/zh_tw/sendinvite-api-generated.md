## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| fromName | string | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'sendInvite 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme-42';
const id: string = 'comment_8f3b21a7';
const fromName: string = 'Elena Morales';
const replyToEmail: string | undefined = undefined;

const result: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName, replyToEmail);
[inline-code-end]

---