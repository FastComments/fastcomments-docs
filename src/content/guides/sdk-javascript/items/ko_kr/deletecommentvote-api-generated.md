## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| voteId | string | 예 |  |
| urlId | string | 예 |  |
| broadcastId | string | 예 |  |
| editKey | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## 예제

[inline-code-attrs-start title = 'deleteCommentVote 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f1b3c";
const commentId: string = "comment_6a7b8c9d";
const voteId: string = "vote_55a1";
const urlId: string = "news/2026/06/19/typescript-updates";
const broadcastId: string = "broadcast_20260619_live_01";
const editKey: string = "edit_3f2a9b";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature";
const result: VoteDeleteResponse = await deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, editKey, sso);
[inline-code-end]

---