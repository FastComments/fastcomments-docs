The individual votes behind one poll's tallies, oldest first.

En afstemning tilhører en kommentar, så stemmer læses altid én afstemning ad gangen - commentId er påkrævet. Det holder hver forespørgsel på de indekser, som samlingen allerede har.

Overholder afstemningens privatliv: en anonym afstemnings stemmer kan ikke læses (poll-anonymous), her eller via id.

## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## Response

Returnerer: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getPollVotes Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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