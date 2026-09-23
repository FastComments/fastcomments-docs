Record a vote on a poll, or move an existing one to a different option. A voter has at most one vote per poll, so calling this again for the same voter moves their vote rather than adding one.

この操作はサイトの投票設定に従います。投票がログインユーザーのみに制限されている場合、anonUserId だけの投票は拒否され、匿名投票は投票ごとに IP アドレス単位でレートリミットが適用されます。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Example

[inline-code-attrs-start title = 'createPollVote の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId is optional and omitted here
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]