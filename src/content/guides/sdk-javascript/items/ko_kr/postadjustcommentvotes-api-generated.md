## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AdjustVotesResponse.ts)

## 예제

[inline-code-attrs-start title = 'postAdjustCommentVotes 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_8f3a2b7d4e";
const adjustCommentVotesParams: AdjustCommentVotesParams = { delta: 1, reason: "useful", source: "web" } as AdjustCommentVotesParams;
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature";
const result: AdjustVotesResponse = await postAdjustCommentVotes(commentId, adjustCommentVotesParams, sso);
[inline-code-end]

---