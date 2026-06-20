## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| voteId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## 範例

[inline-code-attrs-start title = 'deleteModerationVote 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_7f9d3a2b-4e6c-4b1a-9b3d-2a5f8e1c9d0f";
const voteId: string = "vote_91a2b3c4-d5e6-47f8-9a0b-c1d2e3f4a5b6";
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.examplePayload.signature";

const resultWithSso: VoteDeleteResponse = await deleteModerationVote(commentId, voteId, ssoToken);
const resultWithoutSso: VoteDeleteResponse = await deleteModerationVote(commentId, voteId);
[inline-code-end]

---