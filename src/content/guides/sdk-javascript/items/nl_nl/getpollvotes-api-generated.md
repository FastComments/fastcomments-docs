The individuele stemmen achter de tellingen van één poll, oudste eerst.

Een poll behoort tot een commentaar, dus stemmen worden altijd per poll gelezen - commentId is vereist. Dat houdt elke query op de indexen die de collectie al heeft.

Respecteert de privacy van de poll: de stemmen van een anonieme poll kunnen niet worden gelezen (poll-anonymous), hier of via id.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| voterId | string | Nee |  |
| optionId | string | Nee |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getPollVotes voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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