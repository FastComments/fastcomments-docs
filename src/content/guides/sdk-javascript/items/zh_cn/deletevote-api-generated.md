## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| editKey | string | 否 |  |

## 响应

返回: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## 示例

[inline-code-attrs-start title = 'deleteVote 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b21c9';
const id: string = 'vote_4a2d9f1b';
const editKey: string = 'edit_92b7c6a1';

const resultWithoutEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id);
const resultWithEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
[inline-code-end]

---