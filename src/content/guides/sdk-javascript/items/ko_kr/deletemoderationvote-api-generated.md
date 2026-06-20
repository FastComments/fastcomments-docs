## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| voteId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## 예제

[inline-code-attrs-start title = 'deleteModerationVote 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_7f9d3a2b-4e6c-4b1a-9b3d-2a5f8e1c9d0f";
const voteId: string = "vote_91a2b3c4-d5e6-47f8-9a0b-c1d2e3f4a5b6";
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.examplePayload.signature";

const resultWithSso: VoteDeleteResponse = await deleteModerationVote(commentId, voteId, ssoToken);
const resultWithoutSso: VoteDeleteResponse = await deleteModerationVote(commentId, voteId);
[inline-code-end]

---