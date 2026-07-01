## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| commentId | string | 是 |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | 是 |  |
| broadcastId | string | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`PostAdjustCommentVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostAdjustCommentVotesResponse.ts)

## 示例

[inline-code-attrs-start title = 'postAdjustCommentVotes 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8b7a6d";

const adjustParams: AdjustCommentVotesParams = {
  voteDelta: 1,
  // AdjustCommentVotesParams 所需的其他字段
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