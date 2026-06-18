---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| voteId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 是 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## 示例

[inline-code-attrs-start title = 'deleteCommentVote 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3a2b7c';
const commentId: string = 'cmt-5a1f3d92';
const voteId: string = 'vote-3b9c7e1a';
const urlId: string = 'articles/2026/06/typescript-best-practices';
const broadcastId: string = 'broadcast-77f4d2';
const editKey: string = 'edk-9b2f4c';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature';
const result: DeleteCommentVote200Response = await deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, editKey, sso);
[inline-code-end]

---