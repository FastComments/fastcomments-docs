## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| editKey | string | No |  |

## 响应

返回: [`DeleteVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteVoteResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteVote 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-9876";
  const editKey: string = "edit-key-123";

  const resultWithEdit: DeleteVoteResponse = await deleteVote(tenantId, commentId, editKey);
  const resultWithoutEdit: DeleteVoteResponse = await deleteVote(tenantId, commentId);
})();
[inline-code-end]

---