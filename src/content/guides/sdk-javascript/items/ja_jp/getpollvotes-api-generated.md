The individual votes behind one poll's tallies, oldest first.

1つの投票の集計の背後にある個々の投票を、古いものから順に取得します。

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That
keeps every query on the indexes the collection already has.

投票はコメントに属しているため、投票は常に1つの投票ごとに読み取られます - commentId が必須です。これにより、クエリはコレクションが既に持っているインデックス上で実行されます。

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

投票のプライバシーを遵守します：匿名投票の投票は（poll-anonymous）ここでも ID でも読み取れません。

## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| voterId | string | いいえ |  |
| optionId | string | いいえ |  |
| skip | number | いいえ |  |

## Response

返却: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

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

---