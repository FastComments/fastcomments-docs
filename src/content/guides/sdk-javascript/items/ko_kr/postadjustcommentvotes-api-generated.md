## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | 예 |  |
| broadcastId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AdjustVotesResponse.ts)

## 예시

[inline-code-attrs-start title = 'postAdjustCommentVotes 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_987654321";
const adjustParams: AdjustCommentVotesParams = { voteDelta: 1 };
const broadcastId: string = "brd_112233";
const sso: string = "sso-token-xyz";

const result: AdjustVotesResponse = await postAdjustCommentVotes(
  tenantId,
  commentId,
  adjustParams,
  broadcastId,
  sso
);
[inline-code-end]