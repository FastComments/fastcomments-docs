## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| voteId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 是 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回： [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## 示例

[inline-code-attrs-start title = 'deleteCommentVote 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-87e4fd';
const commentId: string = 'cmt-9a12b3f4';
const voteId: string = 'vote-4f6d21b9';
const urlId: string = 'https://www.acme.com/articles/2026/03/25/how-to-test';
const broadcastId: string = 'broadcast-20260325-01';
const editKey: string = 'editkey-6b7c8d9e';
const sso: string = 'sso-jwt-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';

const response: DeleteCommentVote200Response = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey,
  sso
);
[inline-code-end]

---