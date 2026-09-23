Die einzelnen Stimmen hinter den Auszählungen einer Umfrage, älteste zuerst.

Eine Umfrage gehört zu einem Kommentar, daher werden Stimmen immer eine Umfrage nach der anderen gelesen – commentId ist erforderlich. Das hält jede Abfrage auf den bereits vorhandenen Indexen der Sammlung.

Beachtet die Privatsphäre der Umfrage: Stimmen einer anonymen Umfrage können nicht gelesen werden (poll-anonymous), weder hier noch per ID.

## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## Antwort

Rückgabe: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getPollVotes Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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