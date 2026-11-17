## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| votePollBody | VotePollBody | 예 |  |
| urlId | string | 아니요 |  |
| broadcastId | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환값: [`VotePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VotePollResponse.ts)

## 예제

[inline-code-attrs-start title = 'votePoll 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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