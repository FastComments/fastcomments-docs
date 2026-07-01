## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 回應

返回: [`PostAdjustCommentVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostAdjustCommentVotesResponse.ts)

## 範例

[inline-code-attrs-start title = 'postAdjustCommentVotes 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8b7a6d";

const adjustParams: AdjustCommentVotesParams = {
  voteDelta: 1,
  // 需要 AdjustCommentVotesParams 的其他欄位
};

const broadcastId: string = "brd_20230915";
const tenantId: string = "tenant_42";
const sso: string = "sso-token-abc123";

const result: PostAdjustCommentVotesResponse = await postAdjustCommentVotes(
  commentId,
  adjustParams,
  broadcastId,
  tenantId,
  sso
);
[inline-code-end]

---