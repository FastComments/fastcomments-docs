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
(async () => {
  const tenantId: string = 'tenant_8421';
  const id: string = 'vote_3f9b7c2a';
  const editKey: string = 'edit_7Xk9LpQ';
  const responseWithoutEdit: DeleteCommentVote200Response = await deleteVote(tenantId, id);
  const responseWithEdit: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
  console.log(responseWithoutEdit, responseWithEdit);
})();
[inline-code-end]

---