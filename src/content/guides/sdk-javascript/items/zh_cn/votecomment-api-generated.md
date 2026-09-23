## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| voteBodyParams | VoteBodyParams | Yes |  |
| sessionId | string | No |  |
| sso | string | No |  |

## 响应

返回: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## 示例

[inline-code-attrs-start title = 'voteComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runVote() {
  const tenantId: string = 'tenant_001';
  const commentId: string = 'cmt_987654';
  const urlId: string = 'url_12345';
  const broadcastId: string = 'brd_56789';
  const voteBody: VoteBodyParams = { direction: 'up' };
  const sessionId: string = 'sess_abc123';
  const sso: string = 'sso_token_xyz';
  const response: VoteResponse = await voteComment(
    tenantId,
    commentId,
    urlId,
    broadcastId,
    voteBody,
    sessionId,
    sso
  );
  console.log(response);
}
runVote();
[inline-code-end]