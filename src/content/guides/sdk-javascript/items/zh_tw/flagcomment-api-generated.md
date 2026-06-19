## 參數

| Name | Type | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## 範例

[inline-code-attrs-start title = 'flagComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme-corp_01';
const commentId: string = 'cmt_5f8d7a2b3c4e';
const anonUserId: string = 'anon_9c3a1f0e';
const response: FlagCommentResponse = await flagComment(tenantId, commentId, undefined, anonUserId);
[inline-code-end]

---