## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| votePollBody | VotePollBody | 是 |  |
| urlId | string | 否 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`VotePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VotePollResponse.ts)

## 示例

[inline-code-attrs-start title = 'votePoll 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const commentId: string = 'comment_9f8e7d6c';
const votePollBody: VotePollBody = {
  optionId: 'option_1a2b3c4d'
};
const urlId: string = 'blog-post-fastcomments-review';
const sso: string = 'HMAC-SHA256:5f4dcc3b5aa765d61d8327deb882cf99:user@example.com';

const response: VotePollResponse = await votePoll(tenantId, commentId, votePollBody, urlId, undefined, sso);

if (response.status === 'success') {
  const poll: CommentPoll = response.poll;
  const selectedOption: CommentPollOption | undefined = poll.options.find((option: CommentPollOption) => option.id === votePollBody.optionId);
  console.log(`Voted for: ${selectedOption?.text ?? 'unknown'}, total votes: ${poll.totalVotes ?? 0}`);
}
[inline-code-end]