1つの投票の集計の背後にある個々の投票を、古い順に返します。

投票はコメントに属するため、投票は常に1つの投票(ポール)ずつ読み取ります - commentId が必須です。これにより、すべてのクエリがコレクションが既に持つインデックスを使用します。

投票のプライバシー設定に従います: 匿名投票の投票内容は読み取れません(poll-anonymous)。ここでも、id による取得でも同様です。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| voterId | string | いいえ |  |
| optionId | string | いいえ |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## 例

[inline-code-attrs-start title = 'getPollVotes の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]