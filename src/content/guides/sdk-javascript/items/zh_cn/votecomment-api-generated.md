---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 是 |  |
| voteBodyParams | VoteBodyParams | 是 |  |
| sessionId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`VoteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteCommentResponse.ts)

## 示例

[inline-code-attrs-start title = 'voteComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_9f8e7d6c";
const urlId: string = "url_123456";
const broadcastId: string = "bcast_2024_01";

const voteBodyParams: VoteBodyParams = {
  vote: "up",               // 例如，"up" | "down"
  weight: 1,                // 投票的可选权重
};

const sessionId: string = "sess_abc123def";
const sso: string = "sso_token_xyz";

const result: VoteCommentResponse = await voteComment(
  tenantId,
  commentId,
  urlId,
  broadcastId,
  voteBodyParams,
  sessionId,
  sso
);
[inline-code-end]

---