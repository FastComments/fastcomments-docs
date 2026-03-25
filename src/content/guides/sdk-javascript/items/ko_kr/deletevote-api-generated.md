## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| editKey | string | 아니오 |  |

## 응답

반환: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteVote 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7c3f2b4a";
const voteId: string = "vote_4f8d9a11";
const editKey: string = "edit_2b9f8c";
const resultWithoutKey: DeleteCommentVote200Response = await deleteVote(tenantId, voteId);
const resultWithKey: DeleteCommentVote200Response = await deleteVote(tenantId, voteId, editKey);
[inline-code-end]

---