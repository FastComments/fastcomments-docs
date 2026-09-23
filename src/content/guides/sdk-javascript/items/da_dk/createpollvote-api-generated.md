Record en stemme i en afstemning, eller flyt en eksisterende til en anden mulighed. En vælger har højst én stemme pr. afstemning, så hvis du kalder dette igen for den samme vælger, flyttes deres stemme i stedet for at tilføje en ny.

Dette overholder sidens afstemningsindstillinger: hvis afstemning kun er tilladt for loggede brugere, afvises en stemme kun med et anonUserId, og anonyme stemmer er hastighedsbegrænset per IP per afstemning.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Example

[inline-code-attrs-start title = 'createPollVote Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId er valgfri og udeladt her
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]

---