---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| editKey | string | 아니오 |  |

## 응답

반환: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteVote 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b21c9';
const id: string = 'vote_4a2d9f1b';
const editKey: string = 'edit_92b7c6a1';

const resultWithoutEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id);
const resultWithEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
[inline-code-end]

---