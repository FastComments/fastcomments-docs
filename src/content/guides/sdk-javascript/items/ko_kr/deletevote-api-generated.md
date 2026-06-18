## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| editKey | string | No |  |

## 응답

반환: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteVote 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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