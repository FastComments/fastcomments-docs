Record a vote on a poll, or move an existing one to a different option. A voter has at most one vote per poll, so calling this again for the same voter moves their vote rather than adding one.

Dit registreert een stem op een poll, of verplaatst een bestaande naar een andere optie. Een kiezer heeft maximaal één stem per poll, dus het opnieuw aanroepen voor dezelfde kiezer verplaatst hun stem in plaats van er een toe te voegen.

This obeys the site's poll settings: if voting is set to logged-in users only, a vote with just an anonUserId is rejected, and anonymous votes are rate limited per IP per poll.

Dit volgt de poll‑instellingen van de site: als stemmen is ingesteld op alleen ingelogde gebruikers, wordt een stem met alleen een anonUserId afgewezen, en anonieme stemmen worden per IP per poll beperkt.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Example

[inline-code-attrs-start title = 'createPollVote Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---